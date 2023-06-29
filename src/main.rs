// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod adding_account;
mod components;
mod instance;
mod instances;
mod modrinth_installer;
mod new_instance;
mod pages;
mod style;
mod util;
mod vanilla_installer;

use std::{fs, path::PathBuf};

use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use directories::ProjectDirs;
use iced::{
    executor, futures::TryFutureExt, widget::row, Application, Command, Element, Settings, Theme,
};
use once_cell::sync::Lazy;
use pages::Page;
use rfd::{MessageButtons, MessageDialog, MessageLevel};

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "icy-launcher")
        .unwrap()
        .data_dir()
        .to_path_buf()
});

pub fn main() -> Result<()> {
    if !BASE_DIR.exists() {
        fs::create_dir_all(BASE_DIR.as_path())?;
    }

    App::run(Settings::default())?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    LatestInstance,
    Instances,
    NewInstance,
    VanillaInstaller,
    ModrinthInstaller,
    Settings,
    About,
    Accounts,
    AddingAccount(String, String),
    FullscreenMessage(String),
}

struct App {
    view: View,
    show_navbar: bool,
    instances: util::instances::Instances,
    settings: util::settings::Settings,
    accounts: util::accounts::Accounts,
    account_head: Option<Vec<u8>>,
    vanilla_installer: vanilla_installer::VanillaInstaller,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    ShowNavbar(bool),
    GotUpdate(Result<Option<(String, String)>, String>),
    SettingsMessage(pages::settings::Message),
    OpenURL(String),
    AddAccount,
    Login(String, String),
    AddingAccount(Result<util::accounts::Account, String>),
    SelectAccount(util::accounts::Account),
    GotAccountHead(Result<Vec<u8>, String>),
    VanillaInstallerMessage(vanilla_installer::Message),
    CreatedInstance(Result<util::instances::Instances, String>),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instances = util::instances::Instances::load().unwrap();
        let accounts = util::accounts::Accounts::load().unwrap();
        let settings = util::settings::Settings::load().unwrap();

        let updates_command = if settings.check_for_updates {
            Command::perform(
                util::updater::check_for_updates().map_err(|e| e.to_string()),
                Message::GotUpdate,
            )
        } else {
            Command::none()
        };

        let (head_command, account_head) = match accounts.active {
            Some(ref account) => {
                let account = account.clone();

                (
                    Command::perform(
                        util::accounts::get_head(account).map_err(|e| e.to_string()),
                        Message::GotAccountHead,
                    ),
                    Some(Vec::<u8>::new()),
                )
            }
            None => (Command::none(), None),
        };

        let command = Command::batch(vec![updates_command, head_command]);

        (
            Self {
                view: View::LatestInstance,
                show_navbar: true,
                instances,
                settings,
                accounts,
                account_head,
                vanilla_installer: vanilla_installer::VanillaInstaller::new(),
            },
            command,
        )
    }

    fn title(&self) -> String {
        String::from("Icy Launcher")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let mut ret = Command::none();

        match message {
            Message::ChangeView(view) => {
                if view == View::VanillaInstaller {
                    ret = self
                        .vanilla_installer
                        .update(
                            vanilla_installer::Message::GetVersions,
                            self.instances.clone(),
                        )
                        .map(Message::VanillaInstallerMessage);

                    self.view = view;
                } else {
                    self.view = view;
                }
            }
            Message::ShowNavbar(show) => {
                self.show_navbar = show;
            }
            Message::GotUpdate(result) => match result {
                Ok(update) => {
                    if let Some((version, url)) = update {
                        let dialog = MessageDialog::new()
                            .set_level(MessageLevel::Info)
                            .set_title("Update available")
                            .set_description(&format!("Version {} is available", version))
                            .set_buttons(MessageButtons::OkCancelCustom(
                                "Update".to_string(),
                                "Cancel".to_string(),
                            ));

                        if dialog.show() {
                            open::that(url).unwrap();
                        }
                    }
                }
                Err(e) => {
                    println!("Error checking for updates: {e}");
                }
            },
            Message::SettingsMessage(message) => {
                ret = self.settings.update(message).map(Message::SettingsMessage);
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
            }
            Message::AddAccount => {
                let client = util::accounts::get_client().unwrap();
                let details = util::accounts::get_details(&client).unwrap();

                self.view = View::AddingAccount(
                    details.verification_uri().to_string(),
                    details.user_code().secret().to_string(),
                );
                self.show_navbar = false;

                ret = Command::perform(
                    util::accounts::get_account(client, details).map_err(|e| e.to_string()),
                    Message::AddingAccount,
                )
            }
            Message::Login(url, code) => {
                open::that(url).unwrap();

                // copy code to clipboard
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(code).unwrap();
            }
            Message::AddingAccount(account) => {
                self.show_navbar = true;

                match account {
                    Ok(account) => {
                        self.accounts.add_account(account).unwrap();
                        self.view = View::Accounts;
                    }
                    Err(e) => {
                        self.view = View::FullscreenMessage(e);
                    }
                }
            }
            Message::SelectAccount(account) => {
                self.accounts.set_active_account(account).unwrap();
            }
            Message::GotAccountHead(result) => match result {
                Ok(head) => {
                    self.account_head = Some(head);
                }
                Err(e) => {
                    println!("Error getting account head: {e}")
                }
            },
            Message::VanillaInstallerMessage(message) => {
                if message == vanilla_installer::Message::Create {
                    self.view = View::FullscreenMessage("Creating instance...".to_string());
                    self.show_navbar = false;
                }

                ret = self
                    .vanilla_installer
                    .update(message, self.instances.clone())
                    .map(Message::VanillaInstallerMessage);
            }
            Message::CreatedInstance(result) => {
                self.show_navbar = true;

                match result {
                    Ok(instances) => {
                        self.instances = instances;
                        self.view = View::Instances;
                    }
                    Err(e) => {
                        self.view = View::FullscreenMessage(e);
                    }
                }
            }
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let view = match &self.view {
            View::LatestInstance => instance::view("Latest"),
            View::Instances => instances::view(&self.instances),
            View::NewInstance => new_instance::view(),
            View::VanillaInstaller => self
                .vanilla_installer
                .view()
                .map(Message::VanillaInstallerMessage),
            View::ModrinthInstaller => modrinth_installer::view(),
            View::Settings => self.settings.view().map(Message::SettingsMessage),
            View::About => about::view(),
            View::Accounts => accounts::view(&self.accounts),
            View::AddingAccount(url, code) => adding_account::view(url, code),
            View::FullscreenMessage(message) => components::fullscreen_message::view(message),
        };

        if self.show_navbar {
            let navbar = components::navbar::view(&self.view, &self.account_head);

            row![navbar, view].into()
        } else {
            view.into()
        }
    }
}

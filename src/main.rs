// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod adding_account;
mod components;
mod installer;
mod instance;
mod instances;
mod new_instance;
mod settings;
mod style;
mod util;

use std::{fs, path::PathBuf};

use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use directories::ProjectDirs;
use iced::{
    executor, futures::TryFutureExt, widget::row, Application, Command, Element, Settings, Theme,
};
use once_cell::sync::Lazy;
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
    Installer,
    Settings,
    About,
    Accounts,
    AddingAccount(String, String),
    FullscreenMessage(String),
}

struct App {
    installers: Vec<mlua::Lua>,
    view: View,
    instances: util::instances::Instances,
    settings: util::settings::Settings,
    accounts: util::accounts::Accounts,
    account_head: Option<Vec<u8>>,
    selected_installer: Option<usize>,
    new_instance_name: String,
    available_versions: Vec<util::lua::Version>,
    seleted_version: Option<util::lua::Version>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    GotUpdate(Result<Option<(String, String)>, String>),
    CheckForUpdates(bool),
    SaveSettings,
    OpenURL(String),
    AddAccount,
    Login(String, String),
    AddingAccount(Result<util::accounts::Account, String>),
    SelectAccount(util::accounts::Account),
    GotAccountHead(Result<Vec<u8>, String>),
    SelectInstaller(usize),
    ChangeInstanceName(String),
    SelectVersion(util::lua::Version),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instances = util::instances::Instances::load().unwrap();
        let settings = util::settings::Settings::load().unwrap();
        let accounts = util::accounts::Accounts::load().unwrap();

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
                installers: util::lua::get_installers().unwrap(),
                view: View::LatestInstance,
                instances,
                settings,
                accounts,
                account_head,
                selected_installer: None,
                new_instance_name: String::new(),
                available_versions: Vec::new(),
                seleted_version: None,
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
        match message {
            Message::ChangeView(view) => {
                self.view = view;
                Command::none()
            }
            Message::GotUpdate(result) => {
                match result {
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
                }

                Command::none()
            }
            Message::CheckForUpdates(value) => {
                self.settings.check_for_updates = value;
                Command::none()
            }
            Message::SaveSettings => {
                self.settings.save().unwrap();
                Command::none()
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
                Command::none()
            }
            Message::AddAccount => {
                let client = util::accounts::get_client().unwrap();
                let details = util::accounts::get_details(&client).unwrap();

                self.view = View::AddingAccount(
                    details.verification_uri().to_string(),
                    details.user_code().secret().to_string(),
                );

                Command::perform(
                    util::accounts::get_account(client, details).map_err(|e| e.to_string()),
                    Message::AddingAccount,
                )
            }
            Message::Login(url, code) => {
                open::that(url).unwrap();

                // copy code to clipboard
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(code).unwrap();

                Command::none()
            }
            Message::AddingAccount(account) => {
                match account {
                    Ok(account) => {
                        self.accounts.add_account(account).unwrap();
                        self.view = View::Accounts;
                    }
                    Err(e) => {
                        self.view = View::FullscreenMessage(e);
                    }
                }

                Command::none()
            }
            Message::SelectAccount(account) => {
                self.accounts.set_active_account(account).unwrap();
                Command::none()
            }
            Message::GotAccountHead(result) => {
                match result {
                    Ok(head) => {
                        self.account_head = Some(head);
                    }
                    Err(e) => {
                        println!("Error getting account head: {e}")
                    }
                }

                Command::none()
            }
            Message::SelectInstaller(installer) => {
                self.available_versions = util::lua::get_versions(&self.installers[installer]).unwrap();
                self.selected_installer = Some(installer);
                self.view = View::Installer;
                Command::none()
            }
            Message::ChangeInstanceName(new_name) => {
                self.new_instance_name = new_name;
                Command::none()
            }
            Message::SelectVersion(version) => {
                self.seleted_version = Some(version);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let view = match &self.view {
            View::LatestInstance => instance::view("Latest"),
            View::Instances => instances::view(&self.instances),
            View::NewInstance => new_instance::view(&self.installers),
            View::Installer => installer::view(
                &self.installers[self.selected_installer.unwrap()],
                &self.available_versions,
                self.seleted_version.clone(),
                &self.new_instance_name,
            ),
            View::Settings => settings::view(&self.settings),
            View::About => about::view(),
            View::Accounts => accounts::view(&self.accounts),
            View::AddingAccount(url, code) => adding_account::view(url, code),
            View::FullscreenMessage(message) => components::fullscreen_message::view(message),
        };

        if let View::AddingAccount(_, _) = self.view {
            return view;
        }

        if let View::FullscreenMessage(_) = self.view {
            return view;
        }

        let navbar = components::navbar::view(&self.view, &self.account_head);

        row![navbar, view].into()
    }
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod components;
mod pages;
mod style;
mod util;

use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use iced::{
    executor, futures::TryFutureExt, widget::row, Application, Command, Element, Settings, Theme,
};
use once_cell::sync::Lazy;
use pages::{no_instances::NoInstances, Page};
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
    Status,
    LatestInstance,
    Instances,
    NewInstance,
    VanillaInstaller,
    ModrinthInstaller,
    Settings,
    About,
    Accounts,
    AddingAccount,
}

struct App {
    view: View,
    show_navbar: bool,
    status: pages::status::Status,
    instances: util::instances::Instances,
    settings: util::settings::Settings,
    accounts: util::accounts::Accounts,
    adding_account: pages::adding_account::AddingAccount,
    account_head: Option<Vec<u8>>,
    new_instance: pages::new_instance::NewInstance,
    vanilla_installer: pages::vanilla_installer::VanillaInstaller,
    modrinth_installer: pages::modrinth_installer::ModrinthInstaller,
    about: pages::about::About,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    GotUpdate(Result<Option<(String, String)>, String>),
    SettingsMessage(pages::settings::Message),
    OpenURL(String),
    AccountsMessage(pages::accounts::Message),
    AddingAccountMessage(pages::adding_account::Message),
    GotAccountHead(Result<Vec<u8>, String>),
    VanillaInstallerMessage(pages::vanilla_installer::Message),
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

        let mut updates_command = Command::none();

        #[cfg(feature = "updater")]
        if settings.check_for_updates {
            updates_command = Command::perform(
                util::updater::check_for_updates().map_err(|e| e.to_string()),
                Message::GotUpdate,
            );
        }

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
                status: pages::status::Status::new(),
                instances,
                settings,
                accounts,
                adding_account: pages::adding_account::AddingAccount::new(),
                account_head,
                new_instance: pages::new_instance::NewInstance,
                vanilla_installer: pages::vanilla_installer::VanillaInstaller::new(),
                modrinth_installer: pages::modrinth_installer::ModrinthInstaller,
                about: pages::about::About,
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
                        .update(pages::vanilla_installer::Message::GetVersions)
                        .map(Message::VanillaInstallerMessage);

                    self.view = view;
                } else {
                    self.view = view;
                }
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
                    eprintln!("Error checking for updates: {e}");
                }
            },
            Message::SettingsMessage(message) => {
                ret = self.settings.update(message).map(Message::SettingsMessage);
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
            }
            Message::AccountsMessage(message) => {
                if let pages::accounts::Message::AddAccount((_, details)) = &message {
                    self.adding_account.url = details.verification_uri().to_string();
                    self.adding_account.code = details.user_code().secret().to_string();

                    self.view = View::AddingAccount;
                    self.show_navbar = false;
                }

                if let pages::accounts::Message::AddingAccount(_) = &message {
                    self.view = View::Accounts;
                    self.show_navbar = true;
                }

                ret = self.accounts.update(message).map(Message::AccountsMessage);
            }
            Message::AddingAccountMessage(message) => {
                ret = self
                    .adding_account
                    .update(message)
                    .map(Message::AddingAccountMessage);
            }
            Message::GotAccountHead(result) => match result {
                Ok(head) => {
                    self.account_head = Some(head);
                }
                Err(e) => {
                    eprintln!("Error getting account head: {e}");
                }
            },
            Message::VanillaInstallerMessage(mut message) => {
                // if we're creating an instance, show the status page
                if let pages::vanilla_installer::Message::Create(_) = message {
                    self.view = View::Status;
                    self.status.text = "Creating instance...".to_string();
                    self.show_navbar = false;
                    message =
                        pages::vanilla_installer::Message::Create(Some(self.instances.clone()));
                }

                ret = self
                    .vanilla_installer
                    .update(message)
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
                        eprintln!("Error creating instance: {e}");
                    }
                }
            }
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let latest_instance = match &self.instances.list.get(0) {
            Some(instance) => instance.view(),
            None => NoInstances.view(),
        };

        let view = match &self.view {
            View::Status => self.status.view(),
            View::LatestInstance => latest_instance,
            View::Instances => self.instances.view(),
            View::NewInstance => self.new_instance.view(),
            View::VanillaInstaller => self
                .vanilla_installer
                .view()
                .map(Message::VanillaInstallerMessage),
            View::ModrinthInstaller => self.modrinth_installer.view(),
            View::Settings => self.settings.view().map(Message::SettingsMessage),
            View::About => self.about.view(),
            View::Accounts => self.accounts.view().map(Message::AccountsMessage),
            View::AddingAccount => self
                .adding_account
                .view()
                .map(Message::AddingAccountMessage),
        };

        if self.show_navbar {
            let navbar = components::navbar::view(&self.view, &self.account_head);

            row![navbar, view].into()
        } else {
            view.into()
        }
    }
}

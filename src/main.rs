// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

#![feature(let_chains)]

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
    let dir = ProjectDirs::from("eu", "mq1", "icy-launcher")
        .unwrap()
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&dir).unwrap();

    dir
});

pub static META_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("meta");
    fs::create_dir_all(&dir).unwrap();

    dir
});

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Status,
    Instances,
    Instance(Option<util::instances::Instance>),
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
    GotAccountHead(Result<util::accounts::Account, String>),
    VanillaInstallerMessage(pages::vanilla_installer::Message),
    CreatedInstance(Result<util::instances::Instances, String>),
    OpenInstance(util::instances::Instance),
    Downloading(Result<(Vec<util::DownloadItem>, usize), String>),
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

        let head_command = match accounts.active.clone() {
            Some(account) => {
                Command::perform(
                    async move { account.get_head().map_err(|e| e.to_string()) }
                        .map_err(|e| e.to_string()),
                    Message::GotAccountHead,
                )
            }
            None => Command::none(),
        };

        let command = Command::batch(vec![updates_command, head_command]);

        (
            Self {
                view: View::Instance(instances.list.get(0).cloned()),
                show_navbar: true,
                status: pages::status::Status::new(),
                instances,
                settings,
                accounts,
                adding_account: pages::adding_account::AddingAccount::new(),
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
                Ok(account) => {
                    self.accounts.update_account(&account).unwrap();
                }
                Err(e) => {
                    eprintln!("Error getting account head: {e}");
                }
            },
            Message::VanillaInstallerMessage(message) => {
                // if we're creating an instance, show the status page
                if message == pages::vanilla_installer::Message::Create {
                    let name = self.vanilla_installer.name.clone();
                    let version = self.vanilla_installer.selected_version.clone().unwrap();
                    let version = self.vanilla_installer.versions[version].clone();

                    self.instances
                        .new(name, "vanilla".to_string(), version)
                        .unwrap();

                    self.view = View::Instances;
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
            Message::OpenInstance(instance) => {
                self.view = View::Instance(Some(instance));
            }
            Message::Downloading(result) => match result {
                Ok((mut items, total)) => {
                    if let Some(item) = items.pop() {
                        self.status.progress_bar = true;
                        self.status.progress = total - items.len();
                        self.status.progress_total = total;
                        self.status.text =
                            format!("Downloading... {}%", 100 * self.status.progress / total);

                        ret = Command::perform(
                            async move {
                                let res = util::download_file(&item).await;

                                if let Err(e) = &res {
                                    return Err(e.to_string());
                                } else {
                                    return Ok((items, total));
                                }
                            },
                            Message::Downloading,
                        );
                    } else {
                        self.status.progress_bar = false;
                    }
                }
                Err(e) => {
                    eprintln!("Error downloading items: {e}");
                }
            },
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let view = match &self.view {
            View::Status => self.status.view(),
            View::Instance(instance) => {
                if let Some(instance) = instance {
                    instance.view()
                } else {
                    NoInstances.view()
                }
            }
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
            let navbar = components::navbar::view(
                &self.view,
                &self.accounts.active,
                self.instances.list.get(0).cloned(),
            );

            row![navbar, view].into()
        } else {
            view.into()
        }
    }
}

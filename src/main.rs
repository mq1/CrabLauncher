// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod pages;
mod style;
mod subscriptions;
mod types;
mod util;

use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use iced::{executor, widget::row, Application, Command, Element, Settings, Subscription, Theme};
use once_cell::sync::Lazy;
use pages::{
    about::About, new_instance::NewInstance, no_instances::NoInstances, status::Status, Page,
};
use rfd::{MessageButtons, MessageDialog, MessageLevel};
use types::generic_error::GenericError;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Status(Status),
    Instances,
    // TODO: remove argument and use current_instance
    Instance(Option<util::instances::Instance>),
    NewInstance,
    VanillaInstaller,
    Settings,
    About,
    Accounts,
    Download,
    ModrinthModpacks,
}

struct App {
    view: View,
    show_navbar: bool,
    instances: util::instances::Instances,
    current_instance: Option<util::instances::Instance>,
    settings: util::settings::Settings,
    accounts_page: pages::accounts::AccountsPage,
    vanilla_installer: pages::vanilla_installer::VanillaInstaller,
    modrinth_modpacks_page: pages::modrinth_modpacks::ModrinthModpacksPage,
    download: pages::download::Download,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    GotUpdate(Result<Option<(String, String)>, GenericError>),
    SettingsMessage(pages::settings::Message),
    OpenURL(String),
    AccountsMessage(pages::accounts::Message),
    GotAccountHead(Result<util::accounts::Account, GenericError>),
    VanillaInstallerMessage(pages::vanilla_installer::Message),
    CreatedInstance(Result<util::instances::Instances, GenericError>),
    OpenInstance(util::instances::Instance),
    LaunchInstance(util::instances::Instance),
    DeleteInstance(String),
    DownloadMessage(pages::download::Message),
    ModrinthModpacksMessage(pages::modrinth_modpacks::Message),
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
            updates_command =
                Command::perform(util::updater::check_for_updates(), Message::GotUpdate);
        }

        let head_command = match accounts.active.clone() {
            Some(account) => {
                Command::perform(util::accounts::get_head(account), Message::GotAccountHead)
            }
            None => Command::none(),
        };

        let command = Command::batch(vec![updates_command, head_command]);

        (
            Self {
                view: View::Instance(instances.list.get(0).cloned()),
                show_navbar: true,
                instances,
                current_instance: None,
                settings,
                accounts_page: pages::accounts::AccountsPage::new(accounts),
                vanilla_installer: pages::vanilla_installer::VanillaInstaller::new(),
                modrinth_modpacks_page: pages::modrinth_modpacks::ModrinthModpacksPage::new(),
                download: pages::download::Download::new(),
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
                match view {
                    View::VanillaInstaller => {
                        ret = self
                            .vanilla_installer
                            .update(pages::vanilla_installer::Message::GetVersions)
                            .map(Message::VanillaInstallerMessage);
                    }
                    View::ModrinthModpacks => {
                        ret = self
                            .modrinth_modpacks_page
                            .update(pages::modrinth_modpacks::Message::GetModpacks)
                            .map(Message::ModrinthModpacksMessage);
                    }
                    _ => {}
                }

                self.view = view;
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
                if message == pages::accounts::Message::AddAccount {
                    self.show_navbar = false;
                } else if let pages::accounts::Message::AddingAccount(_) = &message {
                    self.show_navbar = true;
                }

                ret = self
                    .accounts_page
                    .update(message)
                    .map(Message::AccountsMessage);
            }
            Message::GotAccountHead(result) => match result {
                Ok(account) => {
                    self.accounts_page
                        .accounts
                        .update_account(&account)
                        .unwrap();
                }
                Err(e) => {
                    eprintln!("Error getting account head: {e}");
                }
            },
            Message::VanillaInstallerMessage(message) => {
                if message == pages::vanilla_installer::Message::Create {
                    let name = self.vanilla_installer.name.clone();
                    let version = self.vanilla_installer.selected_version.clone().unwrap();
                    let version = self.vanilla_installer.versions[version].clone();
                    let optimize_jvm = self.vanilla_installer.optimize_jvm;
                    let memory = self.vanilla_installer.memory.clone();

                    self.instances
                        .new(name, version, None, optimize_jvm, memory)
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
            Message::LaunchInstance(instance) => {
                self.show_navbar = false;
                self.view = View::Download;
                self.current_instance = Some(instance.clone());

                let items =
                    util::vanilla_installer::download_version(&instance.info.minecraft).unwrap();
                self.download.start(items);
            }
            Message::DeleteInstance(name) => {
                let yes = MessageDialog::new()
                    .set_level(MessageLevel::Warning)
                    .set_title("Delete instance")
                    .set_description(&format!("Are you sure you want to delete {}?", name))
                    .set_buttons(MessageButtons::YesNo)
                    .show();

                if yes {
                    self.instances.delete(&name).unwrap();
                    self.view = View::Instances;
                }
            }
            Message::DownloadMessage(message) => {
                if let pages::download::Message::DownloadProgressed(progress) = &message {
                    if progress == &subscriptions::download::Progress::Finished {
                        println!("Done downloading");
                        println!("Launching instance");

                        if let Some(instance) = self.current_instance.as_ref() {
                            self.view = View::Status(Status {
                                text: "Launching...".to_string(),
                            });

                            let account = self.accounts_page.accounts.active.clone().unwrap();

                            let account = self
                                .accounts_page
                                .accounts
                                .refresh_account(account)
                                .unwrap();

                            instance.launch(account).unwrap();

                            self.view = View::Instance(Some(instance.clone()));
                            self.show_navbar = true;
                        }
                    }
                }

                ret = self.download.update(message).map(Message::DownloadMessage);
            }
            Message::ModrinthModpacksMessage(message) => {
                ret = self
                    .modrinth_modpacks_page
                    .update(message)
                    .map(Message::ModrinthModpacksMessage);
            }
        }

        ret
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.download.subscription().map(Message::DownloadMessage)
    }

    fn view(&self) -> Element<Message> {
        let view = match &self.view {
            View::Status(status) => status.view(),
            View::Instance(instance) => {
                if let Some(instance) = instance {
                    instance.view()
                } else {
                    NoInstances.view()
                }
            }
            View::Instances => self.instances.view(),
            View::NewInstance => NewInstance.view(),
            View::VanillaInstaller => self
                .vanilla_installer
                .view()
                .map(Message::VanillaInstallerMessage),
            View::Settings => self.settings.view().map(Message::SettingsMessage),
            View::About => About.view(),
            View::Accounts => self.accounts_page.view().map(Message::AccountsMessage),
            View::Download => self.download.view().map(Message::DownloadMessage),
            View::ModrinthModpacks => self
                .modrinth_modpacks_page
                .view()
                .map(Message::ModrinthModpacksMessage),
        };

        if self.show_navbar {
            let navbar = components::navbar::view(
                &self.view,
                &self.accounts_page.accounts.active,
                self.instances.list.get(0).cloned(),
            );

            row![navbar, view].into()
        } else {
            view.into()
        }
    }
}

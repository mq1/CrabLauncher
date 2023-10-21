// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::futures::TryFutureExt;
use iced::{clipboard, Command, Subscription};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

use crate::pages::Page;
use crate::types::download::Download;
use crate::types::login::Login;
use crate::types::messages::Message;
use crate::types::modrinth_modpacks::ModrinthModpacks;
use crate::types::vanilla_installer::VanillaInstaller;
use crate::util;
use crate::util::accounts::{Account, Accounts};
use crate::util::instances;
use crate::util::instances::Instance;
use crate::util::settings::Settings;

pub struct Launcher {
    pub name: &'static str,
    pub page: Page,
    pub instances: Vec<Instance>,
    pub settings: Settings,
    pub accounts: Accounts,
    pub login: Login,
    pub offline_account_username: String,
    pub vanilla_installer: VanillaInstaller,
    pub modrinth_modpacks: ModrinthModpacks,
    pub download: Download,
}

fn error_dialog(error: &str) {
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title("Error")
        .set_description(error)
        .set_buttons(MessageButtons::Ok)
        .show();
}

impl Default for Launcher {
    fn default() -> Self {
        let instances = match instances::list() {
            Ok(instances) => instances,
            Err(error) => {
                error_dialog(&error.to_string());
                panic!();
            }
        };

        let settings = match Settings::load() {
            Ok(settings) => settings,
            Err(error) => {
                error_dialog(&error.to_string());
                panic!();
            }
        };

        let accounts = match Accounts::load() {
            Ok(accounts) => accounts,
            Err(error) => {
                error_dialog(&error.to_string());
                panic!();
            }
        };

        Self {
            name: "CrabLauncher",
            page: Page::Instances,
            instances,
            settings,
            accounts,
            login: Login::default(),
            offline_account_username: String::new(),
            vanilla_installer: VanillaInstaller::default(),
            modrinth_modpacks: ModrinthModpacks::default(),
            download: Download::default(),
        }
    }
}

impl Launcher {
    pub fn new() -> (Self, Command<Message>) {
        let launcher = Self::default();
        let mut commands = Vec::new();

        // check for updates
        if cfg!(feature = "updater") && launcher.settings.check_for_updates {
            commands.push(Command::perform(
                util::updater::check_for_updates().map_err(|e| e.to_string()),
                Message::GotUpdate,
            ));
        }

        // fetch account head
        if let Some(account) = &launcher.accounts.active {
            commands.push(Command::perform(
                util::accounts::get_head(account.to_owned()).map_err(|e| e.to_string()),
                Message::GotAccountHead,
            ));
        }

        (launcher, Command::batch(commands))
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => {
                if page == Page::VanillaInstaller {
                    self.vanilla_installer = VanillaInstaller::default();
                    self.page = page;
                    return Command::perform(
                        util::vanilla_installer::get_versions().map_err(|e| e.to_string()),
                        Message::GotVersions,
                    );
                }

                self.page = page;
                Command::none()
            }
            Message::Error(error, fatal) => {
                if fatal {
                    self.page = Page::Error(error.to_string());
                } else {
                    error_dialog(&error);
                }

                Command::none()
            }
            Message::OpenURL(url) => {
                if let Err(error) = open::that(url) {
                    self.update(Message::Error(error.to_string(), false))
                } else {
                    Command::none()
                }
            }
            Message::GotUpdate(Ok(Some((version, url)))) => {
                let result = MessageDialog::new()
                    .set_level(MessageLevel::Info)
                    .set_title("Update available")
                    .set_description(format!("Version {} is available", version))
                    .set_buttons(MessageButtons::OkCancelCustom(
                        "Update".to_string(),
                        "Cancel".to_string(),
                    ))
                    .show();

                if result == MessageDialogResult::Ok {
                    self.update(Message::OpenURL(url))
                } else {
                    Command::none()
                }
            }
            Message::GotUpdate(Ok(None)) => {
                println!("No updates available");
                Command::none()
            }
            Message::GotUpdate(Err(error)) => self.update(Message::Error(error, false)),
            Message::GotAccountHead(Ok(account)) => {
                if let Err(error) = self.accounts.update_account(&account) {
                    return self.update(Message::Error(error.to_string(), false));
                }

                Command::none()
            }
            Message::GotAccountHead(Err(error)) => self.update(Message::Error(error, false)),
            Message::UpdateInstances => match instances::list() {
                Ok(instances) => {
                    self.instances = instances;
                    Command::none()
                }
                Err(error) => self.update(Message::Error(error.to_string(), false)),
            },
            Message::CreatedInstance(Ok(())) => {
                self.page = Page::Instances;
                self.update(Message::UpdateInstances)
            }
            Message::CreatedInstance(Err(error)) => self.update(Message::Error(error, true)),
            Message::LaunchInstance(instance) => {
                if let Some(account) = &self.accounts.active {
                    if let Err(error) = instance.launch(account) {
                        self.update(Message::Error(error.to_string(), true))
                    } else {
                        Command::none()
                    }
                } else {
                    self.update(Message::Error("No account selected".to_string(), false))
                }
            }
            Message::OpenInstanceFolder(instance) => {
                if let Err(error) = instance.open_folder() {
                    self.update(Message::Error(error.to_string(), false))
                } else {
                    Command::none()
                }
            }
            Message::DeleteInstance(instance) => {
                if let Err(error) = instance.delete() {
                    self.update(Message::Error(error.to_string(), true))
                } else {
                    self.update(Message::UpdateInstances)
                }
            }
            Message::GetVersions => Command::perform(
                util::vanilla_installer::get_versions().map_err(|e| e.to_string()),
                Message::GotVersions,
            ),
            Message::GotVersions(Ok(versions)) => {
                self.vanilla_installer.versions = versions;
                Command::none()
            }
            Message::GotVersions(Err(error)) => self.update(Message::Error(error, false)),
            Message::ChangeName(name) => {
                self.vanilla_installer.name = name;
                Command::none()
            }
            Message::SetOptimizeJvm(optimize_jvm) => {
                self.vanilla_installer.optimize_jvm = optimize_jvm;
                Command::none()
            }
            Message::SetMemory(memory) => {
                self.vanilla_installer.memory = memory;
                Command::none()
            }
            Message::SelectVersion(index) => {
                self.vanilla_installer.selected_version = Some(index);
                Command::none()
            }
            Message::CreateInstance => {
                let name = self.vanilla_installer.name.clone();
                let version = self.vanilla_installer.selected_version.unwrap();
                let version = self.vanilla_installer.versions[version].clone();
                let optimize_jvm = self.vanilla_installer.optimize_jvm;
                let memory = self.vanilla_installer.memory.clone();

                if let Err(error) = instances::new(name, version, None, optimize_jvm, memory) {
                    self.update(Message::Error(error.to_string(), true))
                } else {
                    self.page = Page::Instances;
                    self.vanilla_installer = VanillaInstaller::default();
                    self.update(Message::UpdateInstances)
                }
            }
            Message::AddAccount => {
                let client = Accounts::get_client().unwrap();
                let details = Accounts::get_details(&client).unwrap();

                self.login.url = details.verification_uri().to_string();
                self.login.code = details.user_code().secret().to_string();
                self.page = Page::AddingAccount;

                Command::perform(
                    Accounts::get_account(client, details).map_err(|e| e.to_string()),
                    Message::LoggedIn,
                )
            }
            Message::LoggedIn(Ok(account)) => {
                self.login = Login::default();

                if let Err(error) = self.accounts.add_account(account) {
                    self.update(Message::Error(error.to_string(), false))
                } else {
                    self.page = Page::Accounts;
                    Command::none()
                }
            }
            Message::LoggedIn(Err(error)) => {
                self.login = Login::default();
                self.page = Page::Accounts;

                self.update(Message::Error(error, false))
            }
            #[cfg(feature = "offline-accounts")]
            Message::OfflineAccountUsernameChanged(username) => {
                self.offline_account_username = username;
                Command::none()
            }
            #[cfg(feature = "offline-accounts")]
            Message::AddOfflineAccount => {
                let account = Account::new_offline(self.offline_account_username.clone());

                if let Err(error) = self.accounts.add_account(account) {
                    return self.update(Message::Error(error.to_string(), false));
                }

                self.page = Page::Accounts;

                Command::none()
            }
            Message::SelectAccount(account) => {
                if let Err(error) = self.accounts.set_active_account(account) {
                    return self.update(Message::Error(error.to_string(), false));
                }

                Command::none()
            }
            Message::OpenLoginUrl => {
                if let Err(error) = open::that(&self.login.url) {
                    self.update(Message::Error(error.to_string(), false))
                } else {
                    clipboard::write(self.login.code.to_owned())
                }
            }
            Message::RemoveAccount(account) => {
                let result = MessageDialog::new()
                    .set_title("Remove account")
                    .set_description(format!(
                        "Are you sure you want to remove {}?",
                        account.mc_username
                    ))
                    .set_buttons(MessageButtons::YesNo)
                    .show();

                if result == MessageDialogResult::Yes {
                    if let Err(error) = self.accounts.remove_account(&account.mc_id) {
                        return self.update(Message::Error(error.to_string(), false));
                    }
                }

                Command::none()
            }
            Message::SetCheckForUpdates(check_for_updates) => {
                self.settings.check_for_updates = check_for_updates;

                Command::none()
            }
            Message::SaveSettings => {
                if let Err(error) = self.settings.save() {
                    self.update(Message::Error(error.to_string(), false))
                } else {
                    Command::none()
                }
            }
            Message::GetModpacks => Command::perform(
                util::modrinth::search_modpacks("").map_err(|e| e.to_string()),
                Message::GotModpacks,
            ),
            Message::GotModpacks(Ok(projects)) => {
                self.modrinth_modpacks.projects = projects.hits;
                Command::none()
            }
            Message::GotModpacks(Err(error)) => self.update(Message::Error(error, false)),
            Message::DownloadProgressed(progress) => {
                self.download.update(progress);
                Command::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.download.subscription()
    }
}
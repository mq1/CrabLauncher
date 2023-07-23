// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{clipboard, Command, Subscription};
use rfd::{MessageButtons, MessageDialog, MessageLevel};

use crate::pages::Page;
use crate::types::download::Download;
use crate::types::generic_error::GenericError;
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
    pub instances: Result<Vec<Instance>, GenericError>,
    pub settings: Result<Settings, GenericError>,
    pub accounts: Result<Accounts, GenericError>,
    pub login: Login,
    pub offline_account_username: String,
    pub vanilla_installer: VanillaInstaller,
    pub modrinth_modpacks: ModrinthModpacks,
    pub download: Download,
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            name: "CrabLauncher",
            page: Page::LatestInstance,
            instances: instances::list(),
            settings: Settings::load(),
            accounts: Accounts::load(),
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

        let commands = Command::batch(vec![
            launcher.check_for_updates_command(),
            launcher.fetch_account_head_command(),
        ]);

        (
            launcher,
            commands,
        )
    }

    fn check_for_updates_command(&self) -> Command<Message> {
        if cfg!(feature = "updater") {
            if let Ok(settings) = &self.settings {
                if settings.check_for_updates {
                    return Command::perform(util::updater::check_for_updates(), Message::GotUpdate);
                }
            }
        }

        Command::none()
    }

    fn fetch_account_head_command(&self) -> Command<Message> {
        if let Ok(accounts) = &self.accounts {
            if let Some(account) = &accounts.active {
                return Command::perform(util::accounts::get_head(account.to_owned()), Message::GotAccountHead);
            }
        }

        Command::none()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => {
                self.page = page;
                Command::none()
            }
            Message::Error(error, fatal) => {
                if fatal {
                    self.page = Page::Error(error);
                } else {
                    MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Error")
                        .set_description(&format!("{}", error))
                        .set_buttons(MessageButtons::Ok)
                        .show();
                }

                Command::none()
            }
            Message::OpenURL(url) => {
                if let Err(error) = open::that(url) {
                    self.update(Message::Error(error.into(), false))
                } else {
                    Command::none()
                }
            }
            Message::GotUpdate(Ok(Some((version, url)))) => {
                let yes = MessageDialog::new()
                    .set_level(MessageLevel::Info)
                    .set_title("Update available")
                    .set_description(&format!("Version {} is available", version))
                    .set_buttons(MessageButtons::OkCancelCustom(
                        "Update".to_string(),
                        "Cancel".to_string(),
                    ))
                    .show();

                if yes {
                    self.update(Message::OpenURL(url))
                } else {
                    Command::none()
                }
            }
            Message::GotUpdate(Ok(None)) => {
                println!("No updates available");
                Command::none()
            }
            Message::GotUpdate(Err(error)) => {
                self.update(Message::Error(error, false))
            }
            Message::GotAccountHead(Ok(account)) => {
                if let Ok(accounts) = &mut self.accounts {
                    if let Err(error) = accounts.update_account(&account) {
                        return self.update(Message::Error(error, false));
                    }
                }

                Command::none()
            }
            Message::GotAccountHead(Err(error)) => {
                self.update(Message::Error(error, false))
            }
            Message::CreatedInstance(Ok(())) => {
                self.instances = instances::list();
                self.page = Page::LatestInstance;
                Command::none()
            }
            Message::CreatedInstance(Err(error)) => {
                self.page = Page::Error(error);
                Command::none()
            }
            Message::LaunchInstance(instance) => {
                if let Ok(accounts) = &self.accounts {
                    if let Some(account) = &accounts.active {
                        if let Err(error) = instance.launch(account) {
                            return self.update(Message::Error(error, true));
                        }
                    } else {
                        let error = GenericError::Generic("No account selected".to_string());
                        return self.update(Message::Error(error, false));
                    }
                }

                Command::none()
            }
            Message::DeleteInstance(instance) => {
                if let Err(error) = instance.delete() {
                    self.update(Message::Error(error, true))
                } else {
                    self.instances = instances::list();
                    Command::none()
                }
            }
            Message::GetVersions => {
                Command::perform(util::vanilla_installer::get_versions(), Message::GotVersions)
            }
            Message::GotVersions(Ok(versions)) => {
                self.vanilla_installer.versions = versions;
                Command::none()
            }
            Message::GotVersions(Err(error)) => {
                self.update(Message::Error(error, false))
            }
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
                let version = self.vanilla_installer.selected_version.clone().unwrap();
                let version = self.vanilla_installer.versions[version].clone();
                let optimize_jvm = self.vanilla_installer.optimize_jvm;
                let memory = self.vanilla_installer.memory.clone();

                if let Err(error) = instances::new(name, version, None, optimize_jvm, memory) {
                    self.update(Message::Error(error, true))
                } else {
                    self.instances = instances::list();
                    self.page = Page::LatestInstance;
                    self.vanilla_installer = VanillaInstaller::default();
                    Command::none()
                }
            }
            Message::AddAccount => {
                let client = Accounts::get_client().unwrap();
                let details = Accounts::get_details(&client).unwrap();

                self.login.url = details.verification_uri().to_string();
                self.login.code = details.user_code().secret().to_string();
                self.page = Page::AddingAccount;

                Command::perform(
                    Accounts::get_account(client, details),
                    Message::LoggedIn,
                )
            }
            Message::LoggedIn(Ok(account)) => {
                self.login = Login::default();

                if let Ok(accounts) = &mut self.accounts {
                    if let Err(error) = accounts.add_account(account) {
                        return self.update(Message::Error(error, false));
                    }

                    self.page = Page::Accounts;
                }

                Command::none()
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

                if let Ok(accounts) = &mut self.accounts {
                    if let Err(error) = accounts.add_account(account) {
                        return self.update(Message::Error(error, false));
                    }

                    self.page = Page::Accounts;
                }

                Command::none()
            }
            Message::SelectAccount(account) => {
                if let Ok(accounts) = &mut self.accounts {
                    if let Err(error) = accounts.set_active_account(account) {
                        return self.update(Message::Error(error, false));
                    }
                }

                Command::none()
            }
            Message::OpenLoginUrl => {
                if let Err(error) = open::that(&self.login.url) {
                    self.update(Message::Error(error.into(), false))
                } else {
                    clipboard::write(self.login.code.to_owned())
                }
            }
            Message::RemoveAccount(account) => {
                if let Ok(accounts) = &mut self.accounts {
                    let yes = MessageDialog::new()
                        .set_title("Remove account")
                        .set_description(&format!(
                            "Are you sure you want to remove {}?",
                            account.mc_username
                        ))
                        .set_buttons(rfd::MessageButtons::YesNo)
                        .show();

                    if yes {
                        if let Err(error) = accounts.remove_account(&account.mc_id) {
                            return self.update(Message::Error(error, false));
                        }
                    }
                }

                Command::none()
            }
            Message::SetCheckForUpdates(check_for_updates) => {
                if let Ok(settings) = &mut self.settings {
                    settings.check_for_updates = check_for_updates;
                }

                Command::none()
            }
            Message::SaveSettings => {
                if let Ok(settings) = &self.settings {
                    if let Err(error) = settings.save() {
                        return self.update(Message::Error(error, false));
                    }
                }

                Command::none()
            }
            Message::GetModpacks => {
                Command::perform(util::modrinth::search_modpacks(""), Message::GotModpacks)
            }
            Message::GotModpacks(Ok(projects)) => {
                self.modrinth_modpacks.projects = projects.hits;
                Command::none()
            }
            Message::GotModpacks(Err(error)) => {
                self.update(Message::Error(error, false))
            }
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

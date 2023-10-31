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
use lib::accounts::{Account, Accounts};
use lib::instances::Instances;
use lib::settings::Settings;

pub struct Launcher {
    pub name: &'static str,
    pub page: Page,
    pub instances: Instances,
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
        let instances = match Instances::load() {
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
                lib::updater::check_for_updates().map_err(|e| e.to_string()),
                Message::GotUpdate,
            ));
        }

        // fetch account head
        if let Some(account) = &launcher.accounts.active {
            commands.push(Command::perform(
                lib::accounts::get_head(account.to_owned()).map_err(|e| e.to_string()),
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
                        lib::vanilla_installer::get_versions().map_err(|e| e.to_string()),
                        Message::GotVersions,
                    );
                }

                self.page = page;
            }
            Message::Error(error, fatal) => {
                // if debug build, panic
                if cfg!(debug_assertions) {
                    panic!("{}", error);
                }

                if fatal {
                    self.page = Page::Error(error.to_string());
                } else {
                    error_dialog(&error);
                }
            }
            Message::OpenURL(url) => {
                if let Err(error) = open::that(url) {
                    return self.update(Message::Error(error.to_string(), false));
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
                    return self.update(Message::OpenURL(url));
                }
            }
            Message::GotUpdate(Ok(None)) => {
                println!("No updates available");
            }
            Message::GotUpdate(Err(error)) => {
                return self.update(Message::Error(error, false));
            }
            Message::GotAccountHead(Ok(account)) => {
                if let Err(error) = self.accounts.update_account(&account) {
                    return self.update(Message::Error(error.to_string(), false));
                }
            }
            Message::GotAccountHead(Err(error)) => {
                return self.update(Message::Error(error, false));
            }
            Message::CreatedInstance(Ok(())) => {
                self.page = Page::Instances;
            }
            Message::CreatedInstance(Err(error)) => {
                return self.update(Message::Error(error, true));
            }
            Message::LaunchInstance(name) => {
                if let Some(account) = &self.accounts.active {
                    if let Err(error) = self.instances.launch(&name, account) {
                        return self.update(Message::Error(error.to_string(), true));
                    }
                } else {
                    return self.update(Message::Error("No account selected".to_string(), false));
                }
            }
            Message::OpenInstanceFolder(name) => {
                let path = self.instances.get_dir(&name);

                if let Err(error) = open::that(path) {
                    return self.update(Message::Error(error.to_string(), false));
                }
            }
            Message::OpenInstanceConfig(name) => {
                let path = self.instances.get_config_path(&name);

                if let Err(error) = open::that(path) {
                    return self.update(Message::Error(error.to_string(), false));
                }
            }
            Message::DeleteInstance(name) => {
                let result = MessageDialog::new()
                    .set_title("Delete instance")
                    .set_description(format!("Are you sure you want to delete {name}?"))
                    .set_buttons(MessageButtons::YesNo)
                    .show();

                if result == MessageDialogResult::Yes {
                    if let Err(error) = self.instances.delete(&name) {
                        return self.update(Message::Error(error.to_string(), true));
                    }
                }
            }
            Message::GetVersions => {
                return Command::perform(
                    lib::vanilla_installer::get_versions().map_err(|e| e.to_string()),
                    Message::GotVersions,
                );
            }
            Message::GotVersions(Ok(versions)) => {
                self.vanilla_installer.versions = versions;
            }
            Message::GotVersions(Err(error)) => {
                return self.update(Message::Error(error, false));
            }
            Message::ChangeName(name) => {
                self.vanilla_installer.name = name;
            }
            Message::SetOptimizeJvm(optimize_jvm) => {
                self.vanilla_installer.optimize_jvm = optimize_jvm;
            }
            Message::SetMemory(memory) => {
                self.vanilla_installer.memory = memory;
            }
            Message::SelectVersion(index) => {
                self.vanilla_installer.selected_version = Some(index);
            }
            Message::CreateInstance => {
                let name = self.vanilla_installer.name.clone();
                let version = self.vanilla_installer.selected_version.unwrap();
                let version = self.vanilla_installer.versions[version].clone();
                let optimize_jvm = self.vanilla_installer.optimize_jvm;
                let memory = self.vanilla_installer.memory.clone();

                if let Err(error) = self
                    .instances
                    .create(name, version, None, optimize_jvm, memory)
                {
                    return self.update(Message::Error(error.to_string(), true));
                } else {
                    self.page = Page::Instances;
                    self.vanilla_installer = VanillaInstaller::default();
                    //return self.update(Message::UpdateInstances);
                }
            }
            Message::AddAccount => {
                let client = Accounts::get_client().unwrap();
                let details = Accounts::get_details(&client).unwrap();

                self.login.url = details.verification_uri().to_string();
                self.login.code = details.user_code().secret().to_string();
                self.page = Page::AddingAccount;

                return Command::perform(
                    Accounts::get_account(client, details).map_err(|e| e.to_string()),
                    Message::LoggedIn,
                );
            }
            Message::LoggedIn(Ok(account)) => {
                self.login = Login::default();

                if let Err(error) = self.accounts.add_account(account) {
                    return self.update(Message::Error(error.to_string(), false));
                } else {
                    self.page = Page::Accounts;
                }
            }
            Message::LoggedIn(Err(error)) => {
                self.login = Login::default();
                self.page = Page::Accounts;

                return self.update(Message::Error(error, false));
            }
            Message::OfflineAccountUsernameChanged(username) => {
                self.offline_account_username = username;
            }
            Message::AddOfflineAccount => {
                let account = Account::new_offline(self.offline_account_username.clone());

                if let Err(error) = self.accounts.add_account(account) {
                    return self.update(Message::Error(error.to_string(), false));
                } else {
                    self.page = Page::Accounts;
                }
            }
            Message::SelectAccount(account) => {
                if let Err(error) = self.accounts.set_active_account(account) {
                    return self.update(Message::Error(error.to_string(), false));
                }
            }
            Message::OpenLoginUrl => {
                if let Err(error) = open::that(&self.login.url) {
                    return self.update(Message::Error(error.to_string(), false));
                } else {
                    return clipboard::write(self.login.code.to_owned());
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
            }
            Message::SetCheckForUpdates(check_for_updates) => {
                self.settings.check_for_updates = check_for_updates;
            }
            Message::SaveSettings => {
                if let Err(error) = self.settings.save() {
                    return self.update(Message::Error(error.to_string(), false));
                }
            }
            Message::GetModpacks => {
                return Command::perform(
                    lib::modrinth::search_modpacks("").map_err(|e| e.to_string()),
                    Message::GotModpacks,
                );
            }
            Message::GotModpacks(Ok(projects)) => {
                self.modrinth_modpacks.projects = projects.hits;
            }
            Message::GotModpacks(Err(error)) => {
                return self.update(Message::Error(error, false));
            }
            Message::DownloadProgressed(progress) => {
                self.download.update(progress);
            }
        }

        Command::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.download.subscription()
    }
}

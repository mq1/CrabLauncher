// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod download;
mod installers;
mod instances;
mod loading;
mod modrinth_installer;
mod modrinth_modpacks;
mod news;
mod settings;
mod style;
mod subscriptions;
mod vanilla_installer;

use anyhow::Result;
use download::Download;
use iced::{
    executor,
    widget::{button, column, container, row, vertical_space},
    Application, Command, Element, Length, Settings as IcedSettings, Subscription, Theme,
};
use mclib::{
    accounts::AccountsDocument,
    instances::Instance,
    launcher_config::LauncherConfig,
    minecraft_news::News as NewsResponse,
    minecraft_version_manifest::Version as VanillaVersion,
    msa::{Account, AccountId},
};
use modrinth_installer::ModrinthInstaller;
use modrinth_modpacks::ModrinthModpacks;
use native_dialog::{MessageDialog, MessageType};

pub fn main() -> iced::Result {
    IceLauncher::run(IcedSettings::default())
}

struct IceLauncher {
    current_view: View,
    news: Option<Result<NewsResponse, String>>,
    instances: Result<Vec<Instance>>,
    accounts_doc: Result<AccountsDocument>,
    config: Result<LauncherConfig>,
    vanilla_versions: Option<Result<Vec<VanillaVersion>, String>>,
    selected_vanilla_version: Option<VanillaVersion>,
    new_instance_name: String,
    download: Download,
    modrinth_modpacks: ModrinthModpacks,
    modrinth_installer: ModrinthInstaller,
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    VanillaInstaller,
    Accounts,
    News,
    About,
    Settings,
    Loading(String),
    Download,
    Installers,
    ModrinthModpacks,
    ModrinthInstaller,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    OpenURL(String),
    GotUpdates(Result<Option<(String, String)>, String>),
    DownloadEvent(subscriptions::download::Event),

    // News
    OpenNews,
    NewsFetched(Result<NewsResponse, String>),

    // Instances
    RemoveInstance(Instance),
    LaunchInstance(Instance),
    NewInstance,
    RefreshInstances,
    InstanceClosed(Result<(), String>),

    // Accounts
    RefreshAccounts,
    RemoveAccount(Account),
    AddAccount,
    AccountAdded(Result<(), String>),
    AccountSelected(AccountId),

    // Installers
    OpenVanillaInstaller,
    OpenModrinthModpacks,
    NewInstanceNameChanged(String),
    VanillaVersionsFetched(Result<Vec<VanillaVersion>, String>),
    VanillaVersionSelected(VanillaVersion),
    CreateVanillaInstance,
    InstanceCreated(Result<(), String>),

    // Settings
    UpdatesTogglerChanged(bool),
    UpdateJvmTogglerChanged(bool),
    OptimizeJvmTogglerChanged(bool),
    UpdateJvmMemory(String),
    ResetConfig,
    SaveConfig,

    ModrinthModpacksMessage(modrinth_modpacks::Message),
    ModrinthInstallerMessage(modrinth_installer::Message),
}

impl Application for IceLauncher {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let config = mclib::launcher_config::read();

        let check_updates = config.as_ref().unwrap().automatically_check_for_updates
            && cfg!(feature = "check-for-updates");

        let app = Self {
            current_view: View::Instances,
            news: None,
            accounts_doc: mclib::accounts::read(),
            instances: mclib::instances::list(),
            config,
            new_instance_name: String::new(),
            vanilla_versions: None,
            selected_vanilla_version: None,
            download: Download::new(),
            modrinth_modpacks: ModrinthModpacks::new(),
            modrinth_installer: ModrinthInstaller::new(),
        };

        let command = if check_updates {
            Command::perform(
                async { mclib::launcher_updater::check_for_updates().map_err(|e| e.to_string()) },
                Message::GotUpdates,
            )
        } else {
            Command::none()
        };

        (app, command)
    }

    fn title(&self) -> String {
        String::from("🧊 Ice Launcher")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ViewChanged(view) => {
                self.current_view = view;
            }
            Message::OpenNews => {
                self.current_view = View::News;

                if self.news.is_none() {
                    return Command::perform(
                        async { mclib::minecraft_news::fetch(None).map_err(|e| e.to_string()) },
                        Message::NewsFetched,
                    );
                }
            }
            Message::NewsFetched(res) => {
                self.news = Some(res);
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
            }
            Message::RemoveInstance(instance) => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Remove instance")
                    .set_text(&format!(
                        "Are you sure you want to remove {}?",
                        &instance.name
                    ))
                    .show_confirm()
                    .unwrap();

                if yes {
                    instance.remove().unwrap();
                    self.update(Message::RefreshInstances);
                }
            }
            Message::LaunchInstance(instance) => {
                self.current_view = View::Loading(format!("Running {}", instance.name));

                if let Ok(doc) = &self.accounts_doc {
                    if !doc.has_account_selected() {
                        MessageDialog::new()
                            .set_type(MessageType::Warning)
                            .set_title("No account selected")
                            .set_text("Please select an account to launch the game")
                            .show_alert()
                            .unwrap();

                        return Command::none();
                    }

                    return Command::perform(
                        async move { instance.launch().map_err(|e| e.to_string()) },
                        Message::InstanceClosed,
                    );
                }
            }
            Message::NewInstance => {
                self.current_view = View::Installers;
            }
            Message::RefreshInstances => {
                self.instances = mclib::instances::list();
            }
            Message::InstanceClosed(res) => {
                if let Err(e) = res {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text(&e)
                        .show_alert()
                        .unwrap();
                }

                self.current_view = View::Instances;
            }
            Message::NewInstanceNameChanged(name) => {
                self.new_instance_name = name;
            }
            Message::OpenVanillaInstaller => {
                self.current_view = View::VanillaInstaller;

                return Command::perform(
                    async {
                        mclib::minecraft_version_manifest::fetch_versions()
                            .map_err(|e| e.to_string())
                    },
                    Message::VanillaVersionsFetched,
                );
            }
            Message::VanillaVersionsFetched(res) => {
                self.vanilla_versions = Some(res);
            }
            Message::VanillaVersionSelected(version) => {
                self.selected_vanilla_version = Some(version);
            }
            Message::CreateVanillaInstance => {
                if self.new_instance_name.is_empty() {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text("Please enter a name for the instance")
                        .show_alert()
                        .unwrap();

                    return Command::none();
                }

                if self.selected_vanilla_version.is_none() {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text("Please select a version")
                        .show_alert()
                        .unwrap();

                    return Command::none();
                }

                let name = &self.new_instance_name;
                let version = self.selected_vanilla_version.as_ref().unwrap();

                self.current_view = View::Loading(format!("Creating instance {name}"));

                let download_items = mclib::instances::new(name, version).unwrap();
                self.current_view = View::Download;
                self.download.start(download_items);
            }
            Message::InstanceCreated(res) => {
                if let Err(e) = res {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text(&e)
                        .show_alert()
                        .unwrap();
                }

                self.current_view = View::Instances;
                self.update(Message::RefreshInstances);
            }
            Message::RefreshAccounts => {
                self.accounts_doc = mclib::accounts::read();
            }
            Message::RemoveAccount(account) => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Remove account")
                    .set_text(&format!(
                        "Are you sure you want to remove {}?",
                        &account.mc_username
                    ))
                    .show_confirm()
                    .unwrap();

                if yes {
                    if let Ok(ref mut doc) = self.accounts_doc {
                        doc.remove(account).unwrap();
                        self.update(Message::RefreshAccounts);
                    }
                }
            }
            Message::AccountSelected(account) => {
                if let Ok(ref mut doc) = self.accounts_doc {
                    doc.set_active(account).unwrap();
                    self.update(Message::RefreshAccounts);
                }
            }
            Message::AddAccount => {
                self.current_view = View::Loading("Logging in".to_string());

                return Command::perform(
                    async { mclib::accounts::add().map_err(|e| e.to_string()) },
                    Message::AccountAdded,
                );
            }
            Message::AccountAdded(res) => {
                if let Some(err) = res.err() {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error adding account")
                        .set_text(&err)
                        .show_alert()
                        .unwrap();
                }

                self.current_view = View::Accounts;
                self.update(Message::RefreshAccounts);
            }
            Message::GotUpdates(updates) => {
                if let Ok(Some((version, url))) = updates {
                    let yes = MessageDialog::new()
                        .set_type(MessageType::Info)
                        .set_title("Update available")
                        .set_text(&format!("A new version of Ice Launcher is available: {version}, would you like to download it?"))
                        .show_confirm()
                        .unwrap();

                    if yes {
                        open::that(url).unwrap();
                    }
                }
            }
            Message::UpdatesTogglerChanged(enabled) => {
                if let Ok(ref mut config) = self.config {
                    config.automatically_check_for_updates = enabled;
                }
            }
            Message::UpdateJvmTogglerChanged(enabled) => {
                if let Ok(ref mut config) = self.config {
                    config.automatically_update_jvm = enabled;
                }
            }
            Message::OptimizeJvmTogglerChanged(enabled) => {
                if let Ok(ref mut config) = self.config {
                    config.automatically_optimize_jvm_arguments = enabled;
                }
            }
            Message::UpdateJvmMemory(memory) => {
                if let Ok(ref mut config) = self.config {
                    config.jvm_memory = memory;
                }
            }
            Message::ResetConfig => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Reset config")
                    .set_text("Are you sure you want to reset the config?")
                    .show_confirm()
                    .unwrap();

                if yes {
                    self.config = mclib::launcher_config::reset();
                }
            }
            Message::SaveConfig => {
                if let Ok(ref config) = self.config {
                    if let Err(e) = mclib::launcher_config::write(config) {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text(&format!("Failed to save config: {e}"))
                            .show_alert()
                            .unwrap();
                    }
                }
            }
            Message::DownloadEvent(event) => {
                match event {
                    subscriptions::download::Event::Finished => {
                        self.current_view = View::Instances;
                        self.update(Message::RefreshInstances);
                    }
                    _ => {}
                }

                self.download.update(event);
            }
            Message::OpenModrinthModpacks => {
                self.current_view = View::ModrinthModpacks;

                return self
                    .modrinth_modpacks
                    .update(modrinth_modpacks::Message::Fetch)
                    .map(Message::ModrinthModpacksMessage);
            }
            Message::ModrinthModpacksMessage(message) => {
                if let modrinth_modpacks::Message::Selected(hit) = message {
                    self.modrinth_installer.hit = Some(hit.clone());
                    self.current_view = View::ModrinthInstaller;

                    return self
                        .modrinth_installer
                        .update(modrinth_installer::Message::Fetch)
                        .map(Message::ModrinthInstallerMessage);
                }

                return self
                    .modrinth_modpacks
                    .update(message)
                    .map(Message::ModrinthModpacksMessage);
            }
            Message::ModrinthInstallerMessage(message) => {
                return self
                    .modrinth_installer
                    .update(message)
                    .map(Message::ModrinthInstallerMessage);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let navbar = container(
            container(
                column![
                    button("Instances")
                        .on_press(Message::ViewChanged(View::Instances))
                        .width(Length::Fill),
                    button("Accounts")
                        .on_press(Message::ViewChanged(View::Accounts))
                        .width(Length::Fill),
                    button("News")
                        .on_press(Message::OpenNews)
                        .width(Length::Fill),
                    vertical_space(Length::Fill),
                    button("Settings")
                        .on_press(Message::ViewChanged(View::Settings))
                        .width(Length::Fill),
                    button("About")
                        .on_press(Message::ViewChanged(View::About))
                        .width(Length::Fill),
                ]
                .spacing(10)
                .padding(20)
                .width(Length::Units(150)),
            )
            .style(style::card()),
        )
        .padding(10);

        let current_view = match self.current_view {
            View::Instances => instances::view(&self.instances),
            View::VanillaInstaller => vanilla_installer::view(
                &self.new_instance_name,
                &self.vanilla_versions,
                &self.selected_vanilla_version,
            ),
            View::Accounts => accounts::view(&self.accounts_doc),
            View::News => news::view(&self.news),
            View::About => about::view(),
            View::Settings => settings::view(&self.config),
            View::Loading(ref message) => loading::view(message),
            View::Download => self.download.view(),
            View::Installers => installers::view(),
            View::ModrinthModpacks => self
                .modrinth_modpacks
                .view()
                .map(Message::ModrinthModpacksMessage),
            View::ModrinthInstaller => self
                .modrinth_installer
                .view()
                .map(Message::ModrinthInstallerMessage),
        };

        row![navbar, current_view].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.download.subscription()
    }
}

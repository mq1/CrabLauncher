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
    Application, Command, Element, Length, Settings, Subscription, Theme,
};
use mclib::{
    accounts::AccountsDocument,
    instances::Instance,
    launcher_config::LauncherConfig,
    minecraft_news::News as NewsResponse,
    minecraft_version_manifest::Version as VanillaVersion,
    modrinth,
    msa::{Account, AccountId},
};
use rfd::{MessageButtons, MessageDialog, MessageLevel};

pub fn main() -> iced::Result {
    IceLauncher::run(Settings {
        default_font: Some(include_bytes!("../../assets/Inter-roman.ttf")),
        ..Default::default()
    })
}

pub struct InstallerInfo {
    name: String,
    vanilla_versions: Option<Result<Vec<VanillaVersion>, String>>,
    selected_vanilla_version: Option<VanillaVersion>,
    available_modpacks: Option<Result<modrinth::SearchResults, String>>,
    selected_modpack: Option<modrinth::Hit>,
    available_modpack_versions: Option<Result<Vec<modrinth::Version>, String>>,
}

struct IceLauncher {
    current_view: View,
    news: Option<Result<NewsResponse, String>>,
    instances: Result<Vec<Instance>>,
    accounts_doc: Result<AccountsDocument>,
    config: Result<LauncherConfig>,
    installer_info: InstallerInfo,
    download: Download,
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
    RemoveAccount(Account),
    AddAccount,
    AccountAdded(Result<Account, String>),
    AccountSelected(AccountId),

    // Installers
    OpenVanillaInstaller,
    OpenModrinthModpacks,
    NewInstanceNameChanged(String),
    VanillaVersionsFetched(Result<Vec<VanillaVersion>, String>),
    VanillaVersionSelected(VanillaVersion),
    CreateVanillaInstance,
    InstanceCreated(Result<(), String>),
    ModpacksFetched(Result<modrinth::SearchResults, String>),
    ModpackSelected(modrinth::Hit),
    ModpackVersionsFetched(Result<Vec<modrinth::Version>, String>),

    // Settings
    UpdatesTogglerChanged(bool),
    UpdateJvmTogglerChanged(bool),
    OptimizeJvmTogglerChanged(bool),
    UpdateJvmMemory(String),
    ResetConfig,
    SaveConfig,
}

impl Application for IceLauncher {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let config = LauncherConfig::load();

        let check_updates = config.as_ref().unwrap().automatically_check_for_updates
            && cfg!(feature = "check-for-updates");

        let app = Self {
            current_view: View::Instances,
            news: None,
            accounts_doc: AccountsDocument::load(),
            instances: mclib::instances::list(),
            config,
            installer_info: InstallerInfo {
                name: String::new(),
                vanilla_versions: None,
                selected_vanilla_version: None,
                available_modpacks: None,
                selected_modpack: None,
                available_modpack_versions: None,
            },
            download: Download::new(),
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
                    .set_level(MessageLevel::Warning)
                    .set_title("Remove instance")
                    .set_description(&format!(
                        "Are you sure you want to remove {}?",
                        &instance.name
                    ))
                    .set_buttons(MessageButtons::YesNo)
                    .show();

                if yes {
                    instance.remove().unwrap();
                    self.update(Message::RefreshInstances);
                }
            }
            Message::LaunchInstance(instance) => {
                self.current_view = View::Loading(format!("Running {}", instance.name));

                if let Ok(ref mut doc) = self.accounts_doc {
                    match doc.active_account {
                        Some(id) => {
                            let account = doc.get_account(&id).unwrap();

                            return Command::perform(
                                async move { instance.launch(account).map_err(|e| e.to_string()) },
                                Message::InstanceClosed,
                            );
                        }
                        None => {
                            let _ = MessageDialog::new()
                                .set_level(MessageLevel::Error)
                                .set_title("No account selected")
                                .set_description("Please select an account to launch the game")
                                .set_buttons(MessageButtons::Ok)
                                .show();

                            return Command::none();
                        }
                    }
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
                    let _ = MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Error")
                        .set_description(&e)
                        .set_buttons(MessageButtons::Ok)
                        .show();
                }

                self.current_view = View::Instances;
            }
            Message::NewInstanceNameChanged(name) => {
                self.installer_info.name = name;
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
                self.installer_info.vanilla_versions = Some(res);
            }
            Message::VanillaVersionSelected(version) => {
                self.installer_info.selected_vanilla_version = Some(version);
            }
            Message::CreateVanillaInstance => {
                if self.installer_info.name.is_empty() {
                    let _ = MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Error")
                        .set_description("Please enter a name for the instance")
                        .set_buttons(MessageButtons::Ok)
                        .show();

                    return Command::none();
                }

                if self.installer_info.selected_vanilla_version.is_none() {
                    let _ = MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Error")
                        .set_description("Please select a version")
                        .set_buttons(MessageButtons::Ok)
                        .show();

                    return Command::none();
                }

                let name = &self.installer_info.name;
                let version = self
                    .installer_info
                    .selected_vanilla_version
                    .as_ref()
                    .unwrap();

                self.current_view = View::Loading(format!("Creating instance {name}"));

                let download_items = mclib::instances::new(name, version).unwrap();
                self.current_view = View::Download;
                self.download.start(download_items);
            }
            Message::InstanceCreated(res) => {
                if let Err(e) = res {
                    let _ = MessageDialog::new()
                        .set_level(MessageLevel::Error)
                        .set_title("Error")
                        .set_description(&e)
                        .set_buttons(MessageButtons::Ok)
                        .show();
                }

                self.current_view = View::Instances;
                self.update(Message::RefreshInstances);
            }
            Message::RemoveAccount(account) => {
                let yes = MessageDialog::new()
                    .set_level(MessageLevel::Warning)
                    .set_title("Remove account")
                    .set_description(&format!(
                        "Are you sure you want to remove {}?",
                        &account.mc_username
                    ))
                    .set_buttons(MessageButtons::YesNo)
                    .show();

                if yes {
                    if let Ok(ref mut doc) = self.accounts_doc {
                        doc.remove_account(account.mc_id).unwrap();
                    }
                }
            }
            Message::AccountSelected(id) => {
                if let Ok(ref mut doc) = self.accounts_doc {
                    doc.set_active_account(id).unwrap();
                }
            }
            Message::AddAccount => {
                self.current_view = View::Loading("Logging in".to_string());

                return Command::perform(
                    async { mclib::accounts::login().map_err(|e| e.to_string()) },
                    Message::AccountAdded,
                );
            }
            Message::AccountAdded(res) => {
                match res {
                    Ok(account) => {
                        if let Ok(ref mut doc) = self.accounts_doc {
                            doc.add_account(account).unwrap();
                        }
                    }
                    Err(e) => {
                        let _ = MessageDialog::new()
                            .set_level(MessageLevel::Error)
                            .set_title("Error adding account")
                            .set_description(&e)
                            .set_buttons(MessageButtons::Ok)
                            .show();
                    }
                }

                self.current_view = View::Accounts;
            }
            Message::GotUpdates(updates) => {
                if let Ok(Some((version, url))) = updates {
                    let yes = MessageDialog::new()
                        .set_level(MessageLevel::Info)
                        .set_title("Update available")
                        .set_description(&format!("A new version of Ice Launcher is available: {version}, would you like to download it?"))
                        .set_buttons(MessageButtons::YesNo)
                        .show();

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
                    .set_level(MessageLevel::Warning)
                    .set_title("Reset config")
                    .set_description("Are you sure you want to reset the config?")
                    .set_buttons(MessageButtons::YesNo)
                    .show();

                if yes {
                    if let Ok(ref mut config) = self.config {
                        config.reset().unwrap();
                    }
                }
            }
            Message::SaveConfig => {
                if let Ok(ref config) = self.config {
                    if let Err(e) = config.save() {
                        let _ = MessageDialog::new()
                            .set_level(MessageLevel::Error)
                            .set_title("Error")
                            .set_description(&format!("Failed to save config: {e}"))
                            .set_buttons(MessageButtons::Ok)
                            .show();
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

                return Command::perform(
                    async { modrinth::fetch_modpacks().map_err(|e| e.to_string()) },
                    Message::ModpacksFetched,
                );
            }
            Message::ModpacksFetched(res) => {
                self.installer_info.available_modpacks = Some(res);
            }
            Message::ModpackSelected(hit) => {
                self.current_view = View::ModrinthInstaller;
                self.installer_info.selected_modpack = Some(hit.clone());

                return Command::perform(
                    async move { hit.fetch_versions().map_err(|e| e.to_string()) },
                    Message::ModpackVersionsFetched,
                );
            }
            Message::ModpackVersionsFetched(res) => {
                self.installer_info.available_modpack_versions = Some(res);
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
            View::VanillaInstaller => vanilla_installer::view(&self.installer_info),
            View::Accounts => accounts::view(&self.accounts_doc),
            View::News => news::view(&self.news),
            View::About => about::view(),
            View::Settings => settings::view(&self.config),
            View::Loading(ref message) => loading::view(message),
            View::Download => self.download.view(),
            View::Installers => installers::view(),
            View::ModrinthModpacks => modrinth_modpacks::view(&self.installer_info),
            View::ModrinthInstaller => modrinth_installer::view(&self.installer_info),
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

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod download;
mod installers;
mod instances;
mod loading;
mod modrinth_modpacks;
mod new_vanilla_instance;
mod news;
mod settings;
mod style;
mod subscriptions;

use about::About;
use accounts::Accounts;
use anyhow::Result;
use download::Download;
use iced::{
    executor,
    widget::{button, column, container, row, vertical_space},
    Application, Command, Element, Length, Settings as IcedSettings, Subscription, Theme,
};
use installers::Installers;
use instances::Instances;
use mclib::msa::AccountId;
use modrinth_modpacks::ModrinthModpacks;
use native_dialog::{MessageDialog, MessageType};
use new_vanilla_instance::NewVanillaInstance;
use news::News;
use settings::Settings;

pub fn main() -> iced::Result {
    IceLauncher::run(IcedSettings::default())
}

struct IceLauncher {
    current_view: View,
    about: About,
    instances: Instances,
    new_vanilla_instance: NewVanillaInstance,
    accounts: Accounts,
    news: News,
    settings: Settings,
    download: Download,
    installers: Installers,
    modrinth_modpacks: ModrinthModpacks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Instances,
    NewVanillaInstance,
    Accounts,
    News,
    About,
    Settings,
    Loading(String),
    Download,
    Installers,
    ModrinthModpacks,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    FetchedNews(Result<mclib::minecraft_news::News, String>),
    OpenURL(String),
    RemoveInstance(String),
    LaunchInstance(mclib::instances::Instance),
    InstanceClosed(Result<(), String>),
    NewInstanceNameChanged(String),
    FetchedVersions(Result<Vec<mclib::minecraft_version_manifest::Version>, String>),
    VersionSelected(mclib::minecraft_version_manifest::Version),
    CreateInstance,
    InstanceCreated(Result<(), String>),
    RemoveAccount(mclib::msa::Account),
    AddAccount,
    AccountAdded(Result<(), String>),
    AccountSelected(AccountId),
    GotUpdates(Result<Option<(String, String)>, String>),
    UpdatesTogglerChanged(bool),
    UpdateJvmTogglerChanged(bool),
    OptimizeJvmTogglerChanged(bool),
    UpdateJvmMemory(String),
    ResetConfig,
    SaveConfig,
    DownloadEvent(subscriptions::download::Event),
    VanillaSelected,
    GotModrinthModpacks(Result<mclib::modrinth::SearchResults, String>),
}

impl Application for IceLauncher {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let settings = Settings::new();

        let check_updates = settings
            .config
            .as_ref()
            .unwrap()
            .automatically_check_for_updates
            && cfg!(feature = "check-for-updates");

        let app = Self {
            current_view: View::Instances,
            about: About::new(),
            accounts: Accounts::new(),
            instances: Instances::new(),
            new_vanilla_instance: NewVanillaInstance::new(),
            news: News::new(),
            settings,
            download: Download::new(),
            installers: Installers::new(),
            modrinth_modpacks: ModrinthModpacks::new(),
        };

        let command = if check_updates {
            Command::perform(check_for_updates(), Message::GotUpdates)
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
                self.current_view = view.clone();

                if view == View::News && self.news.news.is_none() {
                    return Command::perform(News::fetch(), Message::FetchedNews);
                }

                if view == View::NewVanillaInstance
                    && self.new_vanilla_instance.available_versions.is_none()
                {
                    return Command::perform(
                        NewVanillaInstance::fetch_versions(),
                        Message::FetchedVersions,
                    );
                }

                if view == View::ModrinthModpacks
                    && self.modrinth_modpacks.available_modpacks.is_none()
                {
                    return Command::perform(
                        ModrinthModpacks::fetch_modpacks(),
                        Message::GotModrinthModpacks,
                    );
                }
            }
            Message::FetchedNews(news) => {
                self.news.news = Some(news);
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
            }
            Message::RemoveInstance(instance) => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Remove instance")
                    .set_text(&format!("Are you sure you want to remove {}?", instance))
                    .show_confirm()
                    .unwrap();

                if yes {
                    mclib::instances::remove(&instance).unwrap();
                    self.instances.refresh();
                }
            }
            Message::LaunchInstance(instance) => {
                if !self.accounts.has_account_selected() {
                    MessageDialog::new()
                        .set_type(MessageType::Warning)
                        .set_title("No account selected")
                        .set_text("Please select an account to launch the game")
                        .show_alert()
                        .unwrap();

                    return Command::none();
                }

                self.current_view = View::Loading(format!("Launching {}", instance.name));

                return Command::perform(Instances::launch(instance), Message::InstanceClosed);
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
                self.new_vanilla_instance.name = name;
            }
            Message::FetchedVersions(versions) => {
                self.new_vanilla_instance.available_versions = Some(versions);
            }
            Message::VersionSelected(version) => {
                self.new_vanilla_instance.selected_version = Some(version);
            }
            Message::CreateInstance => {
                if self.new_vanilla_instance.name.is_empty() {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text("Please enter a name for the instance")
                        .show_alert()
                        .unwrap();

                    return Command::none();
                }

                if self.new_vanilla_instance.selected_version.is_none() {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text("Please select a version")
                        .show_alert()
                        .unwrap();

                    return Command::none();
                }

                let name = &self.new_vanilla_instance.name;
                let version = self.new_vanilla_instance.selected_version.as_ref().unwrap();

                self.current_view = View::Loading(format!("Creating instance {}", name));

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
                self.instances.refresh();
            }
            Message::RemoveAccount(account) => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Remove account")
                    .set_text(&format!(
                        "Are you sure you want to remove {}?",
                        account.mc_username
                    ))
                    .show_confirm()
                    .unwrap();

                if yes {
                    mclib::accounts::remove(account).unwrap();
                    self.accounts.refresh();
                }
            }
            Message::AccountSelected(account) => {
                mclib::accounts::set_active(account).unwrap();
                self.accounts.refresh();
            }
            Message::AddAccount => {
                async fn add_account() -> Result<(), String> {
                    mclib::accounts::add().map_err(|e| e.to_string())
                }

                self.current_view = View::Loading("Logging in...".to_string());

                return Command::perform(add_account(), Message::AccountAdded);
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
                self.accounts.refresh();
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
                let mut config = self.settings.config.as_mut().unwrap();
                config.automatically_check_for_updates = enabled;
            }
            Message::UpdateJvmTogglerChanged(enabled) => {
                let mut config = self.settings.config.as_mut().unwrap();
                config.automatically_update_jvm = enabled;
            }
            Message::OptimizeJvmTogglerChanged(enabled) => {
                let mut config = self.settings.config.as_mut().unwrap();
                config.automatically_optimize_jvm_arguments = enabled;
            }
            Message::UpdateJvmMemory(memory) => {
                println!("Set memory to {}", memory);
                let mut config = self.settings.config.as_mut().unwrap();
                config.jvm_memory = memory;
            }
            Message::ResetConfig => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Reset config")
                    .set_text("Are you sure you want to reset the config?")
                    .show_confirm()
                    .unwrap();

                if yes {
                    mclib::launcher_config::reset().unwrap();
                    self.settings.refresh();
                }
            }
            Message::SaveConfig => {
                mclib::launcher_config::write(self.settings.config.as_ref().unwrap()).unwrap();
            }
            Message::DownloadEvent(event) => {
                match event {
                    subscriptions::download::Event::Finished => {
                        self.current_view = View::Instances;
                        self.instances.refresh();
                    }
                    _ => {}
                }

                self.download.update(event);
            }
            Message::VanillaSelected => {
                self.current_view = View::NewVanillaInstance;
            }
            Message::GotModrinthModpacks(modpacks) => {
                self.modrinth_modpacks.available_modpacks = Some(modpacks);
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
                        .on_press(Message::ViewChanged(View::News))
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
            View::Instances => self.instances.view(),
            View::NewVanillaInstance => self.new_vanilla_instance.view(),
            View::Accounts => self.accounts.view(),
            View::News => self.news.view(),
            View::About => self.about.view(),
            View::Settings => self.settings.view(),
            View::Loading(ref message) => loading::view(message),
            View::Download => self.download.view(),
            View::Installers => self.installers.view(),
            View::ModrinthModpacks => self.modrinth_modpacks.view(),
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

async fn check_for_updates() -> Result<Option<(String, String)>, String> {
    mclib::launcher_updater::check_for_updates().map_err(|e| e.to_string())
}

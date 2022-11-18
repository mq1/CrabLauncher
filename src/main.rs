// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod instances;
mod lib;
mod loading;
mod new_instance;
mod news;
mod settings;
mod style;

use anyhow::Result;
use arrayvec::ArrayString;
use iced::{
    executor,
    widget::{button, column, container, row, vertical_space},
    Application, Command, Element, Length, Settings, Theme,
};
use native_dialog::{MessageDialog, MessageType};

pub fn main() -> iced::Result {
    IceLauncher::run(Settings::default())
}

struct IceLauncher {
    current_view: View,
    instances: Result<Vec<lib::instances::Instance>>,
    new_instance_name: String,
    available_minecraft_versions:
        Option<Result<Vec<lib::minecraft_version_manifest::Version>, String>>,
    selected_minecraft_version: Option<lib::minecraft_version_manifest::Version>,
    accounts_document: Result<lib::accounts::AccountsDocument>,
    news: Option<Result<lib::minecraft_news::News, String>>,
    config: Result<lib::launcher_config::LauncherConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    Instances,
    NewInstance,
    Accounts,
    News,
    About,
    Settings,
    Loading(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    FetchedNews(Result<lib::minecraft_news::News, String>),
    OpenURL(String),
    RemoveInstance(String),
    LaunchInstance(lib::instances::Instance),
    InstanceClosed(Result<(), String>),
    NewInstanceNameChanged(String),
    FetchedVersions(Result<Vec<lib::minecraft_version_manifest::Version>, String>),
    VersionSelected(lib::minecraft_version_manifest::Version),
    RemoveAccount(lib::msa::Account),
    AddAccount,
    AccountAdded(Result<(), String>),
    AccountSelected(ArrayString<32>),
    GotUpdates(Result<Option<(String, String)>, String>),
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
        (
            Self {
                current_view: View::Instances,
                instances: lib::instances::list(),
                new_instance_name: "".to_string(),
                available_minecraft_versions: None,
                selected_minecraft_version: None,
                accounts_document: lib::accounts::read(),
                news: None,
                config: lib::launcher_config::read(),
            },
            Command::perform(check_for_updates(), Message::GotUpdates),
        )
    }

    fn title(&self) -> String {
        String::from("🧊 Ice Launcher")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ViewChanged(view) => {
                async fn fetch_news() -> Result<lib::minecraft_news::News, String> {
                    lib::minecraft_news::fetch(None).map_err(|e| e.to_string())
                }

                async fn fetch_versions(
                ) -> Result<Vec<lib::minecraft_version_manifest::Version>, String> {
                    lib::minecraft_version_manifest::fetch_versions().map_err(|e| e.to_string())
                }

                self.current_view = view.clone();

                if view == View::News && self.news.is_none() {
                    return Command::perform(fetch_news(), Message::FetchedNews);
                }

                if view == View::NewInstance && self.available_minecraft_versions.is_none() {
                    return Command::perform(fetch_versions(), Message::FetchedVersions);
                }
            }
            Message::FetchedNews(news) => {
                self.news = Some(news);
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
                    lib::instances::remove(&instance).unwrap();
                    self.instances = lib::instances::list();
                }
            }
            Message::LaunchInstance(instance) => {
                async fn launch(instance: lib::instances::Instance) -> Result<(), String> {
                    lib::instances::launch(instance).map_err(|e| e.to_string())
                }

                return Command::perform(launch(instance), Message::InstanceClosed);
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
            Message::FetchedVersions(versions) => {
                self.available_minecraft_versions = Some(versions);
            }
            Message::VersionSelected(version) => {
                self.selected_minecraft_version = Some(version);
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
                    lib::accounts::remove(account).unwrap();
                    self.accounts_document = lib::accounts::read();
                }
            }
            Message::AccountSelected(account) => {
                lib::accounts::set_active(account).unwrap();
                self.accounts_document = lib::accounts::read();
            }
            Message::AddAccount => {
                async fn add_account() -> Result<(), String> {
                    lib::accounts::add().map_err(|e| e.to_string())
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
                self.accounts_document = lib::accounts::read();
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
                let mut config = self.config.as_mut().unwrap();
                config.automatically_check_for_updates = enabled;
            }
            Message::UpdateJvmTogglerChanged(enabled) => {
                let mut config = self.config.as_mut().unwrap();
                config.automatically_update_jvm = enabled;
            }
            Message::OptimizeJvmTogglerChanged(enabled) => {
                let mut config = self.config.as_mut().unwrap();
                config.automatically_optimize_jvm_arguments = enabled;
            }
            Message::UpdateJvmMemory(memory) => {
                println!("Set memory to {}", memory);
                let mut config = self.config.as_mut().unwrap();
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
                    let config = lib::launcher_config::LauncherConfig::default();
                    lib::launcher_config::write(&config).unwrap();
                    self.config = lib::launcher_config::read();
                }
            }
            Message::SaveConfig => {
                lib::launcher_config::write(self.config.as_ref().unwrap()).unwrap();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let navbar = container(
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
        .style(style::card());

        let current_view = match self.current_view {
            View::Instances => instances::view(&self.instances),
            View::NewInstance => new_instance::view(
                &self.new_instance_name,
                &self.available_minecraft_versions,
                &self.selected_minecraft_version,
            ),
            View::Accounts => accounts::view(&self.accounts_document),
            View::News => news::view(&self.news),
            View::About => about::view(),
            View::Settings => settings::view(&self.config),
            View::Loading(ref message) => loading::view(message),
        };

        row![navbar, current_view].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

async fn check_for_updates() -> Result<Option<(String, String)>, String> {
    lib::launcher_updater::check_for_updates().map_err(|e| e.to_string())
}

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

use about::About;
use anyhow::Result;
use download::Download;
use iced::{
    executor,
    widget::{button, column, container, row, vertical_space},
    Application, Command, Element, Length, Settings as IcedSettings, Subscription, Theme,
};
use installers::Installers;
use loading::Loading;
use mclib::{
    accounts::AccountsDocument,
    instances::Instance,
    minecraft_news::News as NewsResponse,
    msa::{Account, AccountId},
};
use modrinth_installer::ModrinthInstaller;
use modrinth_modpacks::ModrinthModpacks;
use native_dialog::{MessageDialog, MessageType};
use settings::Settings;
use vanilla_installer::VanillaInstaller;

pub fn main() -> iced::Result {
    IceLauncher::run(IcedSettings::default())
}

struct IceLauncher {
    current_view: View,
    news: Option<Result<NewsResponse, String>>,
    instances: Result<Vec<Instance>>,
    accounts_doc: Result<AccountsDocument>,
    about: About,
    vanilla_installer: VanillaInstaller,
    settings: Settings,
    download: Download,
    installers: Installers,
    modrinth_modpacks: ModrinthModpacks,
    modrinth_installer: ModrinthInstaller,
    loading: Loading,
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    VanillaInstaller,
    Accounts,
    News,
    About,
    Settings,
    Loading,
    Download,
    Installers,
    ModrinthModpacks,
    ModrinthInstaller,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    OpenURL(String),

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

    OpenVanillaInstaller,
    VanillaInstallerMessage(vanilla_installer::Message),
    InstanceCreated(Result<(), String>),
    GotUpdates(Result<Option<(String, String)>, String>),
    SettingsMessage(settings::Message),
    DownloadEvent(subscriptions::download::Event),
    OpenModrinthModpacks,
    ModrinthModpacksMessage(modrinth_modpacks::Message),
    ModrinthInstallerMessage(modrinth_installer::Message),
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
            news: None,
            about: About::new(),
            accounts_doc: mclib::accounts::read(),
            instances: mclib::instances::list(),
            vanilla_installer: VanillaInstaller::new(),
            settings,
            download: Download::new(),
            installers: Installers::new(),
            modrinth_modpacks: ModrinthModpacks::new(),
            modrinth_installer: ModrinthInstaller::new(),
            loading: Loading::new(),
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
                self.loading.message = format!("Running {}", instance.name);
                self.current_view = View::Loading;

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
            Message::OpenVanillaInstaller => {
                self.current_view = View::VanillaInstaller;

                return self
                    .vanilla_installer
                    .update(vanilla_installer::Message::FetchVersions)
                    .map(Message::VanillaInstallerMessage);
            }
            Message::VanillaInstallerMessage(message) => {
                if let vanilla_installer::Message::CreateInstance = message {
                    if self.vanilla_installer.name.is_empty() {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text("Please enter a name for the instance")
                            .show_alert()
                            .unwrap();

                        return Command::none();
                    }

                    if self.vanilla_installer.selected_version.is_none() {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text("Please select a version")
                            .show_alert()
                            .unwrap();

                        return Command::none();
                    }

                    let name = &self.vanilla_installer.name;
                    let version = self.vanilla_installer.selected_version.as_ref().unwrap();

                    self.loading.message = format!("Creating instance {}", name);
                    self.current_view = View::Loading;

                    let download_items = mclib::instances::new(name, version).unwrap();
                    self.current_view = View::Download;
                    self.download.start(download_items);
                }

                return self
                    .vanilla_installer
                    .update(message)
                    .map(Message::VanillaInstallerMessage);
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
                    self.accounts_doc.as_mut().unwrap().remove(account).unwrap();
                }
            }
            Message::AccountSelected(account) => {
                self.accounts_doc.as_mut().unwrap().set_active(account).unwrap();
            }
            Message::AddAccount => {
                self.loading.message = "Logging in".to_string();
                self.current_view = View::Loading;

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
            Message::SettingsMessage(message) => {
                self.settings.update(message);
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
            View::VanillaInstaller => self
                .vanilla_installer
                .view()
                .map(Message::VanillaInstallerMessage),
            View::Accounts => accounts::view(&self.accounts_doc),
            View::News => news::view(&self.news),
            View::About => self.about.view(),
            View::Settings => self.settings.view().map(Message::SettingsMessage),
            View::Loading => self.loading.view(),
            View::Download => self.download.view(),
            View::Installers => self.installers.view(),
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

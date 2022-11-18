// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod instances;
mod lib;
mod loading;
mod news;
mod style;

use arrayvec::ArrayString;
use color_eyre::Result;
use iced::{
    executor,
    widget::{button, column, container, row, vertical_space},
    Application, Command, Element, Length, Settings, Theme,
};
use native_dialog::{MessageDialog, MessageType};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    IceLauncher::run(Settings::default())?;

    Ok(())
}

struct IceLauncher {
    current_view: View,
    instances: Result<Vec<lib::instances::Instance>>,
    accounts_document: Result<lib::accounts::AccountsDocument>,
    news: Option<Result<lib::minecraft_news::News, String>>,
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    Accounts,
    News,
    About,
    Loading(String),
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    OpenNews,
    FetchedNews(Result<lib::minecraft_news::News, String>),
    OpenURL(String),
    RemoveInstance(String),
    RemoveAccount(lib::msa::Account),
    AddAccount,
    AccountAdded(Result<(), String>),
    AccountSelected(ArrayString<32>),
    GotUpdates(Result<Option<(String, String)>, String>),
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
                accounts_document: lib::accounts::read(),
                news: None,
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
                self.current_view = view;
            }
            Message::OpenNews => {
                self.current_view = View::News;

                if self.news.is_none() {
                    return Command::perform(fetch_news(), Message::FetchedNews);
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
                    .on_press(Message::OpenNews)
                    .width(Length::Fill),
                vertical_space(Length::Fill),
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
            View::Accounts => accounts::view(&self.accounts_document),
            View::News => news::view(&self.news),
            View::About => about::view(),
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

async fn fetch_news() -> Result<lib::minecraft_news::News, String> {
    lib::minecraft_news::fetch(None).map_err(|e| e.to_string())
}

async fn add_account() -> Result<(), String> {
    lib::accounts::add().map_err(|e| e.to_string())
}

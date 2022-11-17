// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod instances;
mod lib;
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
    instances_view: instances::InstancesView,
    accounts_view: accounts::AccountsView,
    news_view: news::NewsView,
    about_view: about::AboutView,
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    Accounts,
    News,
    About,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    OpenNews,
    FetchedNews(Result<lib::minecraft_news::News, String>),
    OpenURL(String),
    RemoveInstance(String),
    RemoveAccount(lib::msa::Account),
    AccountSelected(ArrayString<32>),
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
                instances_view: instances::InstancesView::new(),
                accounts_view: accounts::AccountsView::new(),
                news_view: news::NewsView::new(),
                about_view: about::AboutView::new(),
            },
            Command::none(),
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

                if self.news_view.news.is_none() {
                    return Command::perform(fetch_news(), Message::FetchedNews);
                }
            }
            Message::FetchedNews(news) => {
                self.news_view.news = Some(news);
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
                    self.instances_view.instances = lib::instances::list();
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
                    self.accounts_view.document = lib::accounts::read();
                }
            }
            Message::AccountSelected(account) => {
                lib::accounts::set_active(account).unwrap();
                self.accounts_view.document = lib::accounts::read();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let accounts_text = match &self.accounts_view.document {
            Ok(document) => match document.active_account {
                Some(_) => "Accounts",
                None => "Accounts [!!!]",
            },
            Err(_) => "Accounts [???]",
        };

        let navbar = container(
            column![
                button("Instances")
                    .on_press(Message::ViewChanged(View::Instances))
                    .width(Length::Fill),
                button(accounts_text)
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
            View::Instances => self.instances_view.view(),
            View::Accounts => self.accounts_view.view(),
            View::News => self.news_view.view(),
            View::About => self.about_view.view(),
        };

        row![navbar, current_view].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

async fn fetch_news() -> Result<lib::minecraft_news::News, String> {
    lib::minecraft_news::fetch(None).map_err(|e| e.to_string())
}

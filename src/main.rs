// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod instances;
mod lib;
mod news;
mod style;

use color_eyre::Result;
use iced::{
    executor,
    widget::{button, column, container, row, vertical_space},
    Application, Command, Element, Length, Settings, Theme,
};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    IceLauncher::run(Settings::default())?;

    Ok(())
}

struct IceLauncher {
    current_view: View,
    instances_view: instances::InstancesView,
    news_view: news::NewsView,
    about_view: about::AboutView,
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    News,
    About,
}

#[derive(Debug, Clone)]
pub struct FetchError;

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    OpenNews,
    FetchedNews(Result<lib::minecraft_news::News, FetchError>),
    OpenURL(String),
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
                    return Command::perform(
                        async { lib::minecraft_news::fetch(None).map_err(|_| FetchError) },
                        Message::FetchedNews,
                    );
                }
            }
            Message::FetchedNews(news) => {
                self.news_view.news = Some(news);
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
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
            View::News => self.news_view.view(),
            View::About => self.about_view.view(),
        };

        row![navbar, current_view].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

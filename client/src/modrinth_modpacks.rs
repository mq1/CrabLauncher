// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, Column},
    Command, Element, Length,
};
use mclib::modrinth::{self, Hit, SearchResults};

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    Fetch,
    Fetched(Result<SearchResults, String>),
    Selected(Hit),
}

pub struct ModrinthModpacks {
    pub available_modpacks: Option<Result<SearchResults, String>>,
}

impl ModrinthModpacks {
    pub fn new() -> Self {
        Self {
            available_modpacks: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Fetch => {
                return Command::perform(
                    async { modrinth::fetch_modpacks().map_err(|e| e.to_string()) },
                    Message::Fetched,
                );
            }
            Message::Fetched(modpacks) => {
                self.available_modpacks = Some(modpacks);
            }
            _ => {}
        }

        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Modrinth Modpacks").size(50);

        let content: Element<_> = match &self.available_modpacks {
            Some(Ok(modpacks)) => {
                let mut column = Column::new().spacing(10);
                for modpack in &modpacks.hits {
                    let version_text =
                        text(format!("[Latest Version: {}]", modpack.latest_version));

                    let select_button =
                        button("Select").on_press(Message::Selected(modpack.clone()));

                    let row = row![
                        text(&modpack.title),
                        version_text,
                        horizontal_space(Length::Fill),
                        select_button
                    ]
                    .spacing(10)
                    .padding(10);

                    let container = container(row).style(style::card());

                    column = column.push(container);
                }

                scrollable(column).into()
            }
            Some(Err(error)) => text(error.to_string()).into(),
            None => text("Loading...").into(),
        };

        column![heading, content].spacing(10).padding(20).into()
    }
}

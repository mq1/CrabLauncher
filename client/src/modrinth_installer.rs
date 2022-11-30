// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, container, horizontal_space, row, scrollable, text},
    Element, Length,
};

use crate::{style, Message};

pub struct ModrinthInstaller {
    pub hit: Option<mclib::modrinth::Hit>,
    pub versions: Option<Result<Vec<mclib::modrinth::Version>, String>>,
}

impl ModrinthInstaller {
    pub fn new() -> Self {
        Self {
            hit: None,
            versions: None,
        }
    }

    pub async fn fetch_versions(hit: mclib::modrinth::Hit) -> Result<Vec<mclib::modrinth::Version>, String> {
        hit.fetch_versions().map_err(|e| e.to_string())
    }

    pub fn view(&self) -> Element<Message> {
        let content: Element<_> = match self.hit {
            Some(ref hit) => {
                let heading = text(&hit.title).size(50);

                let content: Element<_> = match &self.versions {
                    Some(Ok(versions)) => {
                        let list = column(
                            versions
                                .iter()
                                .map(|v| {
                                    container(
                                        row![text(&v.name), horizontal_space(Length::Fill)]
                                            .padding(10),
                                    )
                                    .style(style::card())
                                    .into()
                                })
                                .collect(),
                        )
                        .spacing(10);

                        scrollable(list).into()
                    }
                    Some(Err(error)) => text(error).into(),
                    None => text("Loading...").into(),
                };

                column![heading, content].spacing(20).into()
            }
            None => text("Loading...").into(),
        };

        container(content).padding(20).into()
    }
}

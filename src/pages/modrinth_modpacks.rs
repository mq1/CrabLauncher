// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Command, Element, Length, widget::{Column, text, button, horizontal_space, Row, scrollable}};

use crate::{
    pages::Page,
    types::generic_error::GenericError,
    util::{
        self,
        modrinth::{Project, Projects},
    },
};
use crate::components::icons;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    GetModpacks,
    GotModpacks(Result<Projects, GenericError>),
}

pub struct ModrinthModpacksPage {
    pub projects: Vec<Project>,
}

impl ModrinthModpacksPage {
    pub fn new() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

impl Page for ModrinthModpacksPage {
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        let mut ret = Command::none();

        match message {
            Message::GetModpacks => {
                ret = Command::perform(util::modrinth::search_modpacks(""), Message::GotModpacks);
            }
            Message::GotModpacks(Ok(projects)) => {
                self.projects = projects.hits;
            }
            Message::GotModpacks(Err(err)) => {
                eprintln!("Error: {}", err);
            }
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let title = text("Modrinth Modpacks").size(30);

        let mut list = Column::new().spacing(10).padding([0, 20, 0, 0]);
        for project in &self.projects {
            let mut info = Row::new().align_items(Alignment::Center)
                .padding(5)
                .spacing(5)
                .push(text(project.title.to_owned()));

            if !project.display_categories.is_empty() {
                let categories = format!("[{}]", project.display_categories.join(","));

                info = info.push(text(categories));
            }

            info = info
                .push(horizontal_space(Length::Fill))
                .push(icons::view(icons::DOWNLOAD_OUTLINE))
                .push(text(format!("{} Downloads", project.downloads)));

            let button = button(info);

            list = list.push(button);
        }

        let scrollable = scrollable(list).height(Length::Fill);

        Column::new().push(title).push(scrollable).spacing(10).padding(10).into()
    }
}

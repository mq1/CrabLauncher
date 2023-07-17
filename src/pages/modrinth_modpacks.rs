// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, container, text},
    Command, Element, Length,
};

use crate::{
    pages::Page,
    style,
    types::generic_error::GenericError,
    util::{
        self,
        modrinth::{Project, Projects},
    },
};

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

        let mut list = column![].spacing(10);
        for project in &self.projects {
            let title = text(project.title.to_owned());
            let card = container(title)
                .width(Length::Fill)
                .style(style::card())
                .padding(10);
            list = list.push(card);
        }

        column![title, list].spacing(10).padding(10).into()
    }
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::info::Info;
use iced::widget::{text, Row};
use iced::{theme, Color, Element, Sandbox, Settings, Theme};

use crate::instances::Instances;
use crate::message::Message;
use crate::navbar::navbar;
use crate::pages::{Page, PageImpl};

mod icon;
mod info;
mod instances;
mod message;
mod navbar;
mod pages;
mod style;

pub fn main() -> iced::Result {
    Launcher::run(Settings::default())
}

struct Launcher {
    page: Page,
    instances: Instances,
    info: Info,
}

impl Sandbox for Launcher {
    type Message = Message;

    fn new() -> Self {
        Self {
            page: Page::Instances,
            instances: Instances::new(),
            info: Info::new(),
        }
    }

    fn title(&self) -> String {
        String::from("CrabLauncher")
    }

    fn theme(&self) -> Theme {
        Theme::custom(theme::Palette {
            primary: Color::from_rgb8(192, 101, 33),
            ..Theme::Dark.palette()
        })
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangePage(page) => self.page = page,
        }
    }

    fn view(&self) -> Element<Message> {
        let navbar = navbar(&self.page);

        let content = match self.page {
            Page::Instances => self.instances.view(),
            Page::Settings => text("Settings").into(),
            Page::Info => self.info.view(),
        };

        Row::new().push(navbar).push(content).into()
    }
}

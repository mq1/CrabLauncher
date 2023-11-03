// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::{text, Row};
use iced::{executor, theme, Application, Color, Command, Element, Theme};

use crate::info::Info;
use crate::instances::Instances;
use crate::message::Message;
use crate::navbar::navbar;
use crate::pages::Page;
use crate::vanilla_installer::VanillaInstaller;

pub struct Launcher {
    page: Page,
    instances: Instances,
    info: Info,
    vanilla_installer: VanillaInstaller,
}

impl Application for Launcher {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                page: Page::Instances,
                instances: Instances::new(),
                info: Info::new(),
                vanilla_installer: VanillaInstaller::new(),
            },
            Command::none(),
        )
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

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => self.page = page,
            Message::ChangeVanillaInstallerName(name) => self.vanilla_installer.name = name,
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let navbar = navbar(&self.page);

        let content = match self.page {
            Page::Instances => self.instances.view(),
            Page::VanillaInstaller => self.vanilla_installer.view(),
            Page::Settings => text("Settings").into(),
            Page::Info => self.info.view(),
        };

        Row::new().push(navbar).push(content).into()
    }
}

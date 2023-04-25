// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod instances;
mod settings;
mod style;

use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Theme};
use iced_aw::{floating_element::FloatingElement, Wrap};

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    instances: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instances = instances::get_instances();

        (Self { instances }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Icy Launcher")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut instances = Wrap::new();
        for instance in &self.instances {
            let c = container(
                column![
                    text(instance.to_owned()).size(20),
                    button("Edit"),
                    button("Launch"),
                ]
                .align_items(Alignment::Center)
                .spacing(10)
                .padding(10),
            )
            .style(style::card());
            instances = instances.push(container(c).padding(5));
        }

        let content = FloatingElement::new(scrollable(instances).width(Length::Fill), || {
            container(button("New Instance"))
                .padding([0, 20, 10, 0])
                .into()
        });

        column![
            row![
                text("Instances").size(30),
                horizontal_space(Length::Fill),
                button("Accounts"),
                button("Settings"),
            ]
            .spacing(10),
            content
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

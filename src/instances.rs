// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Element, Length,
};
use iced_aw::{FloatingElement, Wrap};

use crate::{icons, style, Message, View};

pub struct Instances {
    list: Vec<String>,
}

impl Instances {
    pub fn load() -> Self {
        let mut instances = Vec::new();

        for i in 1..=100 {
            instances.push(format!("Instance {}", i));
        }

        Self { list: instances }
    }

    pub fn view(&self) -> Element<Message> {
        let mut instances = Wrap::new();
        for instance in &self.list {
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
            container(
                button(
                    text("+")
                        .width(40)
                        .height(40)
                        .size(40)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(style::circle_button()),
            )
            .padding([0, 20, 10, 0])
            .into()
        });

        column![
            row![
                text("Instances").size(30),
                horizontal_space(Length::Fill),
                button("Accounts"),
                button(row![text("Settings"), icons::cog()].spacing(5))
                    .on_press(Message::ChangeView(View::Settings)),
                button("About").on_press(Message::ChangeView(View::About)),
            ]
            .spacing(10),
            content
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

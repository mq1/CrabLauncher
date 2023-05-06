// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::io;

use iced::{
    widget::{button, column, container, horizontal_space, image, row, scrollable, text, Image},
    Alignment, Element, Length,
};
use iced_aw::{FloatingElement, Wrap};

use crate::{accounts::Account, assets, icons, style, Message, View};

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

    pub fn view(
        &self,
        selected_account: &Option<Account>,
    ) -> Element<Message> {
        let mut instances = Wrap::new();
        for instance in &self.list {
            let logo_handle = image::Handle::from_memory(assets::LOGO_PNG);
            let logo = Image::new(logo_handle).height(100);

            let c = container(
                column![
                    logo,
                    text(instance.to_owned()).size(20),
                    button("Edit").style(style::circle_button()),
                    button("Launch").style(style::circle_button()),
                ]
                .align_items(Alignment::Center)
                .spacing(10)
                .padding(10),
            )
            .style(style::card());
            instances = instances.push(container(c).padding(5));
        }

        let content = FloatingElement::new(scrollable(instances).width(Length::Fill), || {
            container(button(icons::plus()).style(style::circle_button()))
                .padding([0, 20, 10, 0])
                .into()
        });

        let account_button = if let Some(account) = selected_account {
            let resp = ureq::get(&format!("https://crafatar.com/avatars/{}", account.mc_id)).call().unwrap();
            let mut bytes = Vec::with_capacity(resp.header("Content-Length").unwrap().parse::<usize>().unwrap());
            io::copy(&mut resp.into_reader(), &mut bytes).unwrap();
            let head_handle = image::Handle::from_memory(bytes);
            let head = Image::new(head_handle).width(50).height(50);

            button(head)
                .style(style::transparent_button())
        } else {
            button(icons::account_alert())
                .style(style::transparent_button())
        };

        column![
            row![
                text("Instances").size(30),
                horizontal_space(Length::Fill),
                account_button,
                button(icons::cog())
                    .style(style::transparent_button())
                    .on_press(Message::ChangeView(View::Settings)),
                button(icons::info())
                    .style(style::transparent_button())
                    .on_press(Message::ChangeView(View::About)),
            ]
            .spacing(10),
            content
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}

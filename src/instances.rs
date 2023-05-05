// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{
        button, column, container, horizontal_space, image, pick_list, row, scrollable, text, Image,
    },
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
        accounts: &Vec<Account>,
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

        let account_picker = pick_list(
            accounts.clone(),
            selected_account.clone(),
            Message::SelectAccount,
        );

        column![
            row![
                text("Instances").size(30),
                horizontal_space(Length::Fill),
                account_picker,
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

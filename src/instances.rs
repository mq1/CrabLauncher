// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::io;

use iced::{
    widget::{button, column, container, horizontal_space, image, row, scrollable, text, Image},
    Alignment, Element, Length,
};
use iced_aw::{FloatingElement, Wrap};

use crate::{assets, icons, style, util, Message, View};

pub fn view<'a>(
    instances: &'a util::instances::Instances,
    active_account: &'a Option<util::accounts::Account>,
) -> Element<'a, Message> {
    let mut wrap: iced_aw::native::Wrap<_, _, iced_aw::native::wrap::direction::Horizontal> =
        Wrap::new();
    for instance in instances {
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
        wrap = wrap.push(container(c).padding(5));
    }

    let content = FloatingElement::new(scrollable(wrap).width(Length::Fill), || {
        container(button(icons::plus()).style(style::circle_button()))
            .padding([0, 20, 10, 0])
            .into()
    });

    let account_button = if let Some(account) = active_account {
        let resp = ureq::get(&format!("https://crafatar.com/avatars/{}", account.mc_id))
            .call()
            .unwrap();
        let mut bytes = Vec::with_capacity(
            resp.header("Content-Length")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
        io::copy(&mut resp.into_reader(), &mut bytes).unwrap();
        let head_handle = image::Handle::from_memory(bytes);
        let head = Image::new(head_handle).width(50).height(50);

        button(head)
    } else {
        button(icons::account_alert())
    };

    column![
        row![
            text("Instances").size(30),
            horizontal_space(Length::Fill),
            account_button
                .style(style::transparent_button())
                .on_press(Message::ChangeView(View::Accounts)),
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

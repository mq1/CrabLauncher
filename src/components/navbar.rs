// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::io;

use iced::{
    widget::{button, column, container, image, vertical_space, Image},
    Element, Length,
};

use crate::{components::icons, style, util, Message, View};

pub fn view<'a>(
    current_view: &'a View,
    active_account: &'a Option<util::accounts::Account>,
) -> Element<'a, Message> {
    let change_view_button = |view: &View| -> Element<Message> {
        let icon = match view {
            View::LatestInstance => icons::cube(),
            View::Instances => icons::grid(),
            View::Accounts => {
                if let Some(account) = active_account {
                    let resp =
                        ureq::get(&format!("https://crafatar.com/avatars/{}", account.mc_id))
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

                    head.into()
                } else {
                    icons::account_alert().into()
                }
            }
            View::Settings => icons::cog(),
            View::About => icons::info(),
        };

        let mut btn = button(icon)
            .style(style::transparent_button())
            .padding(10)
            .on_press(Message::ChangeView(view.clone()));

        if view == current_view {
            btn = btn.style(style::selected_button());
        }

        btn.into()
    };

    let col = column![
        change_view_button(&View::LatestInstance),
        change_view_button(&View::Instances),
        change_view_button(&View::Accounts),
        vertical_space(Length::Fill),
        change_view_button(&View::Settings),
        change_view_button(&View::About),
    ];

    container(col).style(style::dark()).into()
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, image, text, vertical_space, Image},
    Alignment, Element, Length,
};
use iced_aw::Spinner;

use crate::{components::icons, style, Message, View};

pub fn view<'a>(current_view: &'a View, account_head: &'a Option<Vec<u8>>) -> Element<'a, Message> {
    let change_view_button = |view: &View| -> Element<Message> {
        let icon = match view {
            View::LatestInstance => icons::package(),
            View::Instances => icons::grid(),
            View::NewVanillaInstance => icons::minecraft(),
            View::NewModrinthInstance => icons::modrinth(),
            View::Accounts => {
                if let Some(head) = account_head {
                    if head.is_empty() {
                        Spinner::new().into()
                    } else {
                        let head_handle = image::Handle::from_memory(head.clone());
                        let head = Image::new(head_handle).width(32).height(32);

                        head.into()
                    }
                } else {
                    icons::account_alert().into()
                }
            }
            View::Settings => icons::cog(),
            View::About => icons::info(),
            View::AddingAccount(_, _) => todo!(),
            View::FullscreenMessage(_) => todo!(),
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
        vertical_space(Length::Fill),
        text("New"),
        vertical_space(5),
        change_view_button(&View::NewVanillaInstance),
        change_view_button(&View::NewModrinthInstance),
        vertical_space(Length::Fill),
        change_view_button(&View::Accounts),
        change_view_button(&View::Settings),
        change_view_button(&View::About),
    ]
    .align_items(Alignment::Center);

    container(col).style(style::dark()).into()
}

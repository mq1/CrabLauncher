// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, image, text, vertical_space, Image},
    Alignment, Element, Length,
};
use iced_aw::Spinner;

use crate::{components::icons, style, Message, View};

pub fn view<'a>(current_view: &'a View, account_head: &'a Option<Vec<u8>>) -> Element<'a, Message> {
    let change_view_button = |view: View, icon: Element<'static, Message>| -> Element<Message> {
        button(icon)
            .padding(10)
            .style(if &view == current_view {
                style::selected_button()
            } else {
                style::transparent_button()
            })
            .on_press(Message::ChangeView(view))
            .into()
    };

    let account_icon = if let Some(head) = account_head {
        if head.is_empty() {
            Spinner::new().into()
        } else {
            let head_handle = image::Handle::from_memory(head.clone());
            let head = Image::new(head_handle).width(32).height(32);

            head.into()
        }
    } else {
        icons::account_alert().into()
    };

    let col = column![
        change_view_button(View::LatestInstance, icons::package()),
        change_view_button(View::Instances, icons::grid()),
        vertical_space(Length::Fill),
        text("Install"),
        vertical_space(5),
        change_view_button(View::NewVanillaInstance, icons::minecraft()),
        change_view_button(View::NewModrinthInstance, icons::modrinth()),
        vertical_space(Length::Fill),
        change_view_button(View::Accounts, account_icon),
        change_view_button(View::Settings, icons::cog()),
        change_view_button(View::About, icons::info()),
    ]
    .align_items(Alignment::Center);

    container(col).style(style::dark()).into()
}

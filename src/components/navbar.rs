// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    theme,
    widget::{button, column, container, image, tooltip, vertical_space, Image},
    Alignment, Element, Length,
};
use iced_aw::Spinner;

use crate::{components::icons, style, Message, View};

pub fn view<'a>(current_view: &'a View, account_head: &'a Option<Vec<u8>>) -> Element<'a, Message> {
    let change_view_button =
        |view: View, icon: Element<'static, Message>, tooltip_text| -> Element<Message> {
            tooltip(
                button(icon)
                    .padding(10)
                    .style(if &view == current_view {
                        style::selected_button()
                    } else {
                        style::transparent_button()
                    })
                    .on_press(Message::ChangeView(view)),
                tooltip_text,
                tooltip::Position::Right,
            )
            .gap(10)
            .style(theme::Container::Box)
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
        change_view_button(View::LatestInstance, icons::package(), "Latest Instance"),
        change_view_button(View::NewInstance, icons::package_plus(), "New Instance"),
        change_view_button(View::Instances, icons::grid(), "Instances"),
        vertical_space(Length::Fill),
        change_view_button(View::Accounts, account_icon, "Accounts"),
        change_view_button(View::Settings, icons::cog(), "Settings"),
        change_view_button(View::About, icons::info(), "About Icy Launcher"),
    ]
    .align_items(Alignment::Center);

    container(col).style(style::dark()).into()
}

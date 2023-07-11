// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    theme,
    widget::{button, column, container, image, tooltip, vertical_space, Image},
    Alignment, Element, Length,
};
use iced_aw::Spinner;

use crate::{components::icons, style, util, Message, View};

pub fn view<'a>(
    current_view: &'a View,
    active_account: &'a Option<util::accounts::Account>,
    latest_instance: Option<util::instances::Instance>,
) -> Element<'a, Message> {
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
    
    let account_icon = if let Some(account) = active_account {
        if account.cached_head.is_empty() {
            Spinner::new().into()
        } else {
            let head_handle = image::Handle::from_memory(account.cached_head.clone());
            let head = Image::new(head_handle).width(32).height(32);

            head.into()
        }
    } else {
        icons::account_alert().into()
    };

    let col = column![
        change_view_button(
            View::Instance(latest_instance),
            icons::package(),
            "Latest Instance"
        ),
        change_view_button(View::NewInstance, icons::package_plus(), "New Instance"),
        change_view_button(View::Instances, icons::grid(), "Instances"),
        vertical_space(Length::Fill),
        change_view_button(View::Accounts, account_icon, "Accounts"),
        change_view_button(View::Settings, icons::cog(32.), "Settings"),
        change_view_button(View::About, icons::info(), "About Icy Launcher"),
    ]
    .align_items(Alignment::Center);

    container(col).style(style::dark()).into()
}

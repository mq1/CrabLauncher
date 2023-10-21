// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element,
    Length, theme, widget::{button, Column, container, tooltip, vertical_space},
};
use iced::widget::image;
use iced_aw::Spinner;

use crate::{components::icons, Message, style};
use crate::pages::Page;
use crate::util::accounts::Accounts;

fn change_view_button<'a>(
    page: Page,
    current_page: &Page,
    icon: Element<'static, Message>,
    tooltip_text: &str,
) -> Element<'a, Message> {
    let style = if page == *current_page {
        style::selected_button()
    } else {
        theme::Button::Text
    };

    tooltip(
        button(icon)
            .padding(10)
            .style(style)
            .on_press(Message::ChangePage(page)),
        tooltip_text,
        tooltip::Position::Right,
    )
        .gap(10)
        .style(theme::Container::Box)
        .into()
}

pub fn view<'a>(
    launcher_name: &'a str,
    current_page: &'a Page,
    accounts: &'a Accounts,
) -> Element<'a, Message> {
    let account_icon = {
        if let Some(account) = &accounts.active {
            if let Some(cached_head) = account.cached_head.to_owned() {
                let handle = image::Handle::from_memory(cached_head);

                image(handle)
                    .width(32)
                    .height(32)
                    .into()
            } else {
                Spinner::new().into()
            }
        } else {
            icons::view_custom(icons::ACCOUNT_ALERT_OUTLINE, 32)
        }
    };

    let col = Column::new()
        .push(change_view_button(
            Page::Instances,
            current_page,
            icons::view_custom(icons::VIEW_GRID_OUTLINE, 32),
            "Instances",
        ))
        .push(change_view_button(
            Page::NewInstance,
            current_page,
            icons::view_custom(icons::VIEW_GRID_PLUS_OUTLINE, 32),
            "New Instance",
        ))
        .push(vertical_space(Length::Fill))
        .push(change_view_button(
            Page::Accounts,
            current_page,
            account_icon,
            "Accounts",
        ))
        .push(change_view_button(
            Page::Settings,
            current_page,
            icons::view_custom(icons::COG_OUTLINE, 32),
            "Settings",
        ))
        .push(change_view_button(
            Page::About,
            current_page,
            icons::view_custom(icons::INFORMATION_OUTLINE, 32),
            &format!("About {}", launcher_name),
        ))
        .align_items(Alignment::Center);

    container(col).style(style::dark()).into()
}

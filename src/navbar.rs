// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::icon::Icon;
use crate::message::Message;
use crate::pages::Page;
use crate::style;
use crate::style::NavbarButtonStyle;
use iced::widget::{button, container, vertical_space, Column};
use iced::{Element, Length};

pub fn navbar(current_page: &Page) -> Element<'static, Message> {
    let navbar = Column::new()
        .push(
            button(Icon::ViewGridOutline.view(32))
                .style(style::navbar_button(if current_page == &Page::Instances {
                    NavbarButtonStyle::Selected
                } else {
                    NavbarButtonStyle::Normal
                }))
                .padding(8)
                .on_press(Message::ChangePage(Page::Instances)),
        )
        .push(
            button(Icon::CogOutline.view(32))
                .style(style::navbar_button(if current_page == &Page::Settings {
                    NavbarButtonStyle::Selected
                } else {
                    NavbarButtonStyle::Normal
                }))
                .padding(8)
                .on_press(Message::ChangePage(Page::Settings)),
        )
        .push(vertical_space(Length::Fill))
        .push(
            button(Icon::InformationOutline.view(32))
                .style(style::navbar_button(if current_page == &Page::Info {
                    NavbarButtonStyle::Selected
                } else {
                    NavbarButtonStyle::Normal
                }))
                .padding(8)
                .on_press(Message::ChangePage(Page::Info)),
        );

    let navbar = container(navbar)
        .height(Length::Fill)
        .style(style::navbar_container());

    navbar.into()
}

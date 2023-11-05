// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::image::Handle;
use iced::widget::{text, vertical_space, Column, Image};
use iced::{Alignment, Length};

pub const LOGO_PNG: &[u8] = include_bytes!("../assets/logo-128x128.png");
pub struct Info {
    icon: &'static [u8],
    name: String,
    version: String,
    author: String,
    license: String,
}

impl Info {
    pub fn new() -> Self {
        Self {
            icon: LOGO_PNG,
            name: "CrabLauncher".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            license: env!("CARGO_PKG_LICENSE").to_string(),
        }
    }

    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        let icon = Handle::from_memory(self.icon);

        Column::new()
            .push(vertical_space(Length::Fill))
            .push(Image::new(icon))
            .push(text(&self.name).size(64))
            .push(text(format!("Version: {}", self.version)))
            .push(text(format!("Author: {}", self.author)))
            .push(text(format!("License: {}", self.license)))
            .push(vertical_space(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(8)
            .into()
    }
}

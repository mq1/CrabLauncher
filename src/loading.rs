// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, horizontal_space, row, text, vertical_space},
    Alignment, Element, Length,
};

use crate::Message;

pub struct LoadingView {
    pub message: String,
}

impl LoadingView {
    pub fn new() -> Self {
        Self {
            message: "".to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            vertical_space(Length::Fill),
            row![
                horizontal_space(Length::Fill),
                text(&self.message).size(50),
                horizontal_space(Length::Fill)
            ],
            vertical_space(Length::Fill),
        ]
        .into()
    }
}
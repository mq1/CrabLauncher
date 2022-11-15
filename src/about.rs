// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{widget::text, Element};

use crate::Message;

pub struct AboutView;

impl AboutView {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        text("About").into()
    }
}

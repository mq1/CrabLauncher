// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Command, Element};

pub mod settings;

pub trait Page {
    type Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    fn view(&self) -> Element<Self::Message>;
}

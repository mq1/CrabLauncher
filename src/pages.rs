// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Instances,
    Settings,
    Info,
}

pub trait PageImpl {
    fn view(&self) -> iced::Element<'_, Message>;
    fn update(&mut self, message: Message);
}

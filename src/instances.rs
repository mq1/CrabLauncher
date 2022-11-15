// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, text},
    Element,
};

use crate::Message;

pub struct InstancesView;

impl InstancesView {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        column!(text("Instances")).padding(20).into()
    }
}

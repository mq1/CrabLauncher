// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{widget::text, Command, Element};

use crate::{pages::Page, util::instances::Instance, Message};

impl Page for Instance {
    type Message = Message;

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        text(&self.name).size(30).into()
    }
}

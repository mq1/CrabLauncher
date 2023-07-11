// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Command, Element};

pub mod about;
pub mod accounts;
pub mod instance;
pub mod instances;
pub mod modrinth_installer;
pub mod new_instance;
pub mod no_instances;
pub mod settings;
pub mod status;
pub mod vanilla_installer;

pub trait Page {
    type Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;
    fn view(&self) -> Element<Self::Message>;
}

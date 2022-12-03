// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, text},
    Element,
};

use crate::Message;

pub struct Installers;

impl Installers {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Installers").size(50);

        let vanilla_button = button("Vanilla").on_press(Message::OpenVanillaInstaller);

        let modrinth_button = button("Modrinth").on_press(Message::OpenModrinthModpacks);

        let installers = column![vanilla_button, modrinth_button].spacing(10);

        column![heading, installers].spacing(20).padding(20).into()
    }
}

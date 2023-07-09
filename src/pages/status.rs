// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, progress_bar, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{pages::Page, Message};

#[derive(Default)]
pub struct Status {
    pub text: String,
    pub progress_bar: bool,
    pub progress: usize,
    pub progress_total: usize,
}

impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Page for Status {
    type Message = Message;

    fn update(&mut self, _: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut col = column![vertical_space(Length::Fill), text(&self.text).size(30)]
            .align_items(Alignment::Center)
            .width(Length::Fill);

        if self.progress_bar {
            let bar = progress_bar(0.0..=self.progress_total as f32, self.progress as f32)
                .width(Length::Fill);

            col = col.push(bar);
        }

        col.push(vertical_space(Length::Fill)).into()
    }
}

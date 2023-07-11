// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, progress_bar, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{pages::Page, Message};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Progress {
    pub current: usize,
    pub total: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub text: String,
    pub progress: Option<Progress>,
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

        if let Some(progress) = &self.progress {
            let bar = progress_bar(0.0..=progress.total as f32, progress.current as f32)
                .width(Length::Fill);

            col = col.push(bar);
        }

        col.push(vertical_space(Length::Fill)).into()
    }
}

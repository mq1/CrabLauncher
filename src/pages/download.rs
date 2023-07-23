// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, Length, widget::{Column, progress_bar, text, vertical_space},
};

use crate::types::download::{Download, State};
use crate::types::messages::Message;

pub fn view(download: &Download) -> Element<Message> {
    let current_progress = match &download.state {
        State::Idle { .. } => 0.0,
        State::Downloading { progress, queue: _ } => *progress,
        State::Finished { .. } => 100.0,
        State::Errored { .. } => 0.0,
    };

    let progress_bar = progress_bar(0.0..=100.0, current_progress);

    let current_progress = format!("Downloading... {current_progress:.2}%");
    let text = text(match &download.state {
        State::Idle => "Starting download",
        State::Finished => "Download finished!",
        State::Downloading { .. } => &current_progress,
        State::Errored => "Something went wrong :(",
    });

    Column::new()
        .push(vertical_space(Length::Fill))
        .push(text)
        .push(progress_bar)
        .push(vertical_space(Length::Fill))
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center)
        .into()
}

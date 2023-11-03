// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::icon::Icon;
use iced::widget::{text, vertical_space, Column, Row};
use iced::Alignment;

pub struct Instance {
    pub name: String,
}

pub struct Instances {
    list: Vec<Instance>,
}

impl Instances {
    pub fn new() -> Self {
        let test_list = vec![
            Instance {
                name: String::from("test"),
            },
            Instance {
                name: String::from("test2"),
            },
        ];

        Self { list: test_list }
    }

    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        Column::new()
            .push(vertical_space(48))
            .push(
                Row::new()
                    .push(Icon::ArrowLeft.view(24))
                    .push(text("You don't have any instances yet. Create one!").size(25))
                    .align_items(Alignment::Center)
                    .spacing(10),
            )
            .padding(10)
            .into()
    }
}

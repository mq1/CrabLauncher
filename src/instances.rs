// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::{text, Column};

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
            .push(text("Instances").size(50))
            .push(text("test"))
            .into()
    }
}

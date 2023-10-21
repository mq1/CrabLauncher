// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

pub mod instances;
pub mod new_instance;

#[derive(PartialEq)]
pub enum Page {
    Instances,
    NewInstance,
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Page::Instances => "Instances".to_string(),
            Page::NewInstance => "New Instance".to_string(),
        }
    }
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

pub mod about;
pub mod instances;
pub mod new_instance;

#[derive(PartialEq)]
pub enum Page {
    Instances,
    NewInstance,
    About,
}

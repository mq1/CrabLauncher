// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only
pub mod instances;
pub mod settings;

#[derive(PartialEq)]
pub enum Page {
    Instances,
    Settings,
}

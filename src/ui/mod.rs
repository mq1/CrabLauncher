// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

pub mod accent;
mod info;
mod instances;
mod nav;
pub mod root;
mod settings;

#[derive(PartialEq)]
pub enum View {
    Instances,
    Settings,
}

pub enum Modal {
    None,
    Info,
}

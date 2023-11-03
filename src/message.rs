// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::pages::Page;

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page),
    VanillaInstaller(VanillaInstallerMessage),
}

#[derive(Debug, Clone)]
pub enum VanillaInstallerMessage {
    ChangeName(String),
}

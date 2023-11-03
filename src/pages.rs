// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Instances,
    VanillaInstaller,
    Settings,
    Info,
}

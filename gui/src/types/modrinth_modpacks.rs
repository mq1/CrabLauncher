// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use lib::modrinth::Project;

pub struct ModrinthModpacks {
    pub projects: Vec<Project>,
}

impl Default for ModrinthModpacks {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

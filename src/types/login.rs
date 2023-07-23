// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

pub struct Login {
    pub url: String,
    pub code: String,
}

impl Default for Login {
    fn default() -> Self {
        Self {
            url: String::new(),
            code: String::new(),
        }
    }
}

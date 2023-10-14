// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuel.quarneti@proton.me>
// SPDX-License-Identifier: GPL-2.0-only

use std::fs;
use std::path::PathBuf;

pub struct Instance {
    path: PathBuf,
    pub name: String,
    pub version: String,
}

impl Instance {
    pub fn open_dir(&self) -> std::io::Result<()> {
        open::that(&self.path)
    }

    pub fn delete(&self) -> std::io::Result<()> {
        fs::remove_dir_all(&self.path)
    }
}

impl From<PathBuf> for Instance {
    fn from(path: PathBuf) -> Self {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let version = "placeholder".to_string();

        Self {
            path,
            name,
            version,
        }
    }
}

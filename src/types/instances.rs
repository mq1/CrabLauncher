use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use anyhow::Result;

use crate::BASE_DIR;
// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuel.quarneti@proton.me>
// SPDX-License-Identifier: GPL-2.0-only

use crate::types::instance::Instance;

pub static INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("instances");
    fs::create_dir_all(&dir).unwrap();

    dir
});

pub struct Instances {
    pub list: Vec<Instance>,
}

impl Instances {
    pub fn new() -> Result<Self> {
        let mut list = Vec::new();

        for entry in fs::read_dir(&*INSTANCES_DIR)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                list.push(path.into());
            }
        }

        Ok(Self {
            list,
        })
    }
}

// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Instance {
    path: PathBuf,
}

pub fn list(data_dir: &Path) -> Box<[Instance]> {
    let instances_dir = data_dir.join("instances");

    let mut instances = Vec::new();
    if let Ok(entries) = fs::read_dir(&instances_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    instances.push(Instance { path });
                }
            }
        }
    }

    instances.into_boxed_slice()
}

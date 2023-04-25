// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

pub fn get_instances() -> Vec<String> {
    let mut instances = Vec::new();

    for i in 1..=100 {
        instances.push(format!("Instance {}", i));
    }

    instances
}

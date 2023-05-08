// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

pub type Instances = Vec<String>;

pub trait InstancesExt {
    fn load() -> Self;
}

impl InstancesExt for Instances {
    fn load() -> Self {
        let mut instances = Vec::new();

        for i in 1..=100 {
            instances.push(format!("Instance {}", i));
        }

        instances
    }
}

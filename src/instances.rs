// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, text},
    Element,
};

use crate::{lib, Message};

pub struct InstancesView {
    instances: Vec<lib::instances::Instance>,
}

impl InstancesView {
    pub fn new() -> Self {
        Self {
            instances: lib::instances::list().unwrap(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Instances").size(50);

        let instances_list = column(
            self.instances
                .iter()
                .map(|instance| text(instance.name.clone()).into())
                .collect(),
        );

        column!(heading, instances_list).padding(20).into()
    }
}

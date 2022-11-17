// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text, vertical_space},
    Element, Length,
};

use crate::{lib, style, Message};

pub struct InstancesView {
    pub instances: Result<Vec<lib::instances::Instance>>,
}

impl InstancesView {
    pub fn new() -> Self {
        Self {
            instances: lib::instances::list(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Instances").size(50);

        let instances_list: Element<_> = match &self.instances {
            Ok(instances) => column(
                instances
                    .iter()
                    .map(|instance| {
                        container(
                            row![
                                text(format!(
                                    "{} [{}] [{}]",
                                    instance.name,
                                    instance.info.instance_type,
                                    instance.info.minecraft_version
                                )),
                                horizontal_space(Length::Fill),
                                button("Remove")
                                    .on_press(Message::RemoveInstance(instance.name.clone())),
                                button("Launch"),
                            ]
                            .spacing(10)
                            .padding(10),
                        )
                        .style(style::card())
                        .into()
                    })
                    .collect(),
            )
            .spacing(10)
            .into(),
            Err(_) => text("Failed to load instances").into(),
        };

        column![heading, instances_list]
            .spacing(20)
            .padding(20)
            .into()
    }
}

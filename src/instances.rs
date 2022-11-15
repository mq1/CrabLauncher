// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text, vertical_space},
    Element, Length,
};

use crate::{lib, style, Message};

pub struct InstancesView {
    instances: Result<Vec<lib::instances::Instance>>,
}

impl InstancesView {
    pub fn new() -> Self {
        Self {
            instances: lib::instances::list(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Instances").size(50);

        let instances_list: Element<_> = match self.instances {
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
                                button("Launch")
                            ]
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

        column!(heading, vertical_space(Length::Units(20)), instances_list)
            .padding(20)
            .into()
    }
}

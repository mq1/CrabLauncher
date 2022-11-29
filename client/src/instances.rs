// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text},
    Element, Length,
};

use crate::{style, Message, View};

pub struct Instances {
    list: Result<Vec<mclib::instances::Instance>>,
}

impl Instances {
    pub fn new() -> Self {
        Self {
            list: mclib::instances::list(),
        }
    }

    pub fn refresh(&mut self) {
        self.list = mclib::instances::list();
    }

    pub async fn launch(instance: mclib::instances::Instance) -> Result<(), String> {
        mclib::instances::launch(instance).map_err(|e| e.to_string())
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Instances").size(50);

        let instances_list: Element<_> = match &self.list {
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
                                button("Launch")
                                    .on_press(Message::LaunchInstance(instance.clone())),
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

        let new_instance_button =
            button("New instance").on_press(Message::ViewChanged(View::Installers));

        column![heading, instances_list, new_instance_button]
            .spacing(20)
            .padding(20)
            .into()
    }
}

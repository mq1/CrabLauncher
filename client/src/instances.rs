// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text},
    Element, Length,
};
use mclib::instances::Instance;

use crate::{icons, style, Message};

pub fn view(instances: &Result<Vec<Instance>>) -> Element<Message> {
    let heading = row![icons::blocks().size(50), text("Instances").size(50)].spacing(5);

    let instances_list: Element<_> = match instances {
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
                            button(row![text("Delete"), icons::delete()].spacing(5))
                                .on_press(Message::RemoveInstance(instance.clone())),
                            button(row![text("Launch"), icons::rocket()].spacing(5))
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

    let new_instance_button = button("New instance").on_press(Message::NewInstance);

    column![heading, instances_list, new_instance_button]
        .spacing(20)
        .padding(20)
        .into()
}

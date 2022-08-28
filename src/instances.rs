// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::instances::{InstanceInfo, InstanceType},
    AppState,
};

fn get_instance_icon(instance_type: &InstanceType) -> String {
    match instance_type {
        InstanceType::Vanilla => "🍦".to_string(),
        InstanceType::Fabric => "🧵".to_string(),
        InstanceType::Forge => "🔥".to_string(),
    }
}

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🧊 Instances").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(|(_, info): &(_, InstanceInfo), _env: &_| {
                            get_instance_icon(&info.instance_type)
                        }))
                        .with_default_spacer()
                        .with_child(Label::new(|(name, _): &(String, _), _env: &_| {
                            name.to_string()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Launch 🚀"))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::instances),
            )
            .vertical(),
        )
        .with_flex_spacer(1.)
        .with_child(Flex::row().with_flex_spacer(1.).with_child(Label::new(
            |data: &AppState, _env: &_| match &data.active_account {
                Some(account) => format!("Active account: {}", account.1.minecraft_username),
                None => "No active account".to_string(),
            },
        )))
        .padding(10.)
}

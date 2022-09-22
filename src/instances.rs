// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::{
        self,
        instances::{InstanceInfo, InstanceType},
    },
    AppState, View,
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
                        .with_child(Button::new("Delete ❌").on_click(remove_instance))
                        .with_default_spacer()
                        .with_child(Button::new("Launch 🚀").on_click(launch_instance))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::instances),
            )
            .vertical(),
        )
        .with_default_spacer()
        .with_child(
            Button::new("New Instance ✨").on_click(|_, data: &mut AppState, _| {
                data.current_view = View::InstanceTypeSelection;
            }),
        )
        .with_flex_spacer(1.)
        .with_child(Flex::row().with_flex_spacer(1.).with_child(Label::new(
            |data: &AppState, _env: &_| match &data.active_account {
                Some(entry) => format!("Active account: {}", entry.account.minecraft_username),
                None => "No active account".to_string(),
            },
        )))
        .padding(10.)
}

fn remove_instance(
    ctx: &mut druid::EventCtx,
    (instance_name, _): &mut (String, InstanceInfo),
    _env: &druid::Env,
) {
    let instance_name = instance_name.clone();
    smol::spawn(lib::instances::remove(instance_name.clone())).detach();

    let event_sink = ctx.get_external_handle();
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.instances.retain(|(name, _)| name != &instance_name);
    });
}

fn launch_instance(
    ctx: &mut druid::EventCtx,
    (instance_name, _): &mut (String, InstanceInfo),
    _env: &druid::Env,
) {
    let instance_name = instance_name.clone();
    let event_sink = ctx.get_external_handle();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        let instance_name = instance_name.clone();
        let account = data.active_account.clone().unwrap().account;
        smol::spawn(lib::instances::launch(instance_name, account)).detach();
    });
}

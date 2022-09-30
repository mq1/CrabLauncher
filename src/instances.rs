// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Target, Widget, WidgetExt,
};

use crate::{
    lib::instances::{Instance, InstanceType},
    AppState, View, LAUNCH_INSTANCE, REMOVE_INSTANCE,
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
                        .with_child(Label::<Instance>::dynamic(|instance, _| {
                            get_instance_icon(&instance.info.instance_type)
                        }))
                        .with_default_spacer()
                        .with_child(Label::<Instance>::dynamic(|instance, _| {
                            instance.name.to_owned()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::<Instance>::new("Delete ❌").on_click(
                            |ctx, instance, _| {
                                ctx.get_external_handle()
                                    .submit_command(
                                        REMOVE_INSTANCE,
                                        instance.to_owned(),
                                        Target::Auto,
                                    )
                                    .expect("Failed to submit command");
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Button::<Instance>::new("Launch 🚀").on_click(
                            |ctx, instance, _| {
                                ctx.get_external_handle()
                                    .submit_command(
                                        LAUNCH_INSTANCE,
                                        instance.to_owned(),
                                        Target::Auto,
                                    )
                                    .expect("Failed to submit command");
                            },
                        ))
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
            Button::<AppState>::new("New Instance ✨").on_click(|_, data, _| {
                data.current_view = View::InstanceTypeSelection;
            }),
        )
        .with_flex_spacer(1.)
        .with_child(
            Flex::row().with_flex_spacer(1.).with_child(
                Button::<AppState>::dynamic(|data, _| match &data.active_account {
                    Some(account) => format!("Active account: {}", account.mc_username),
                    None => "⚠️ No active account".to_string(),
                })
                .on_click(|_, data, _| {
                    data.current_view = View::Accounts;
                }),
            ),
        )
        .padding(10.)
}

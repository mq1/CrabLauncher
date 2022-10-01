// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    lens,
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, LensExt, Widget, WidgetExt,
};

use crate::{
    lib::{
        self,
        instances::{Instance, InstanceType},
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
                        .with_child(Label::<(_, Instance)>::dynamic(|(_, instance), _| {
                            get_instance_icon(&instance.info.instance_type)
                        }))
                        .with_default_spacer()
                        .with_child(Label::<(_, Instance)>::dynamic(|(_, instance), _| {
                            instance.name.to_owned()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(
                            Button::<(Vector<Instance>, Instance)>::new("Delete ❌").on_click(
                                |_, (instances, instance), _| {
                                    tokio::spawn(lib::instances::remove(instance.clone()));
                                    instances.retain(|i| i.name != instance.name);
                                },
                            ),
                        )
                        .with_default_spacer()
                        .with_child(Button::<(_, Instance)>::new("Launch 🚀").on_click(
                            |ctx, (_, instance), _| {
                                let event_sink = ctx.get_external_handle();
                                tokio::spawn(launch_instance(event_sink, instance.clone()));
                            },
                        ))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(lens::Identity.map(
                    |data: &AppState| (data.instances.clone(), data.instances.clone()),
                    |data: &mut AppState, (instances, _)| {
                        data.instances = instances;
                    },
                )),
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

async fn launch_instance(event_sink: druid::ExtEventSink, instance: Instance) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        let account = data.active_account.clone().unwrap();
        tokio::spawn(lib::instances::launch(instance, account));
    });
}

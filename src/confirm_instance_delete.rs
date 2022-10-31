// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, Flex, Label},
    UnitPoint, Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(
            Flex::column()
                .with_child(Label::<AppState>::dynamic(|data, _| {
                    format!(
                        "Are you sure you want to delete {}?",
                        data.selected_instance.as_ref().unwrap().name
                    )
                }))
                .with_default_spacer()
                .with_child(
                    Flex::row()
                        .with_child(Button::<AppState>::new("Cancel ❌").on_click(|_, data, _| {
                            data.current_view = View::Instances;
                        }))
                        .with_default_spacer()
                        .with_child(Button::<AppState>::new("Confirm ✅").on_click(
                            |ctx, data, _| {
                                let event_sink = ctx.get_external_handle();
                                let instance = data.selected_instance.as_ref().unwrap().to_owned();
                                thread::spawn(move || lib::instances::remove(instance, event_sink));
                            },
                        )),
                )
                .align_horizontal(UnitPoint::CENTER)
                .align_vertical(UnitPoint::CENTER),
            1.,
        )
        .padding(10.)
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

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
                    data.current_message.to_owned()
                }))
                .with_default_spacer()
                .with_child(
                    Flex::row()
                        .with_child(Button::<AppState>::new("Cancel ❌").on_click(|_, data, _| {
                            data.current_view = View::Instances;
                        }))
                        .with_default_spacer()
                        .with_child(Button::<AppState>::new("Confirm ✅").on_click(
                            |_, data, _| {
                                let instance = data.selected_instance.as_ref().unwrap().to_owned();
                                data.instances.retain(|i| i.name != instance.name);
                                tokio::spawn(lib::instances::remove(instance));
                                data.current_view = View::Instances;
                            },
                        )),
                )
                .align_horizontal(UnitPoint::CENTER)
                .align_vertical(UnitPoint::CENTER),
            1.,
        )
        .padding(10.)
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, Scroll, Switch, TextBox},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::launcher_config::{self, LauncherConfig},
    AppState,
};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("⚙️ Settings").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_child(Label::new("Automatically check for updates"))
                            .with_flex_spacer(1.)
                            .with_child(
                                Switch::new().lens(LauncherConfig::automatically_check_for_updates),
                            )
                            .padding(5.)
                            .border(Color::GRAY, 1.)
                            .rounded(5.),
                    )
                    .with_default_spacer()
                    .with_child(
                        Flex::row()
                            .with_child(Label::new("JVM arguments"))
                            .with_default_spacer()
                            .with_flex_child(
                                Scroll::new(TextBox::new().lens(LauncherConfig::jvm_arguments))
                                    .horizontal(),
                                1.,
                            )
                            .padding(5.)
                            .border(Color::GRAY, 1.)
                            .rounded(5.),
                    )
                    .with_default_spacer()
                    .with_child(
                        Flex::row()
                            .with_child(Label::new("JVM Memory"))
                            .with_flex_spacer(1.)
                            .with_child(TextBox::new().lens(LauncherConfig::jvm_memory))
                            .padding(5.)
                            .border(Color::GRAY, 1.)
                            .rounded(5.),
                    ),
            )
            .vertical(),
        )
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_flex_spacer(1.)
                .with_child(Button::new("Reset to default settings 🔄").on_click(
                    |_ctx, data: &mut LauncherConfig, _env| {
                        *data = LauncherConfig::default();
                    },
                ))
                .with_default_spacer()
                .with_child(Button::new("Save settings 📝").on_click(
                    |_ctx, data: &mut LauncherConfig, _env| {
                        let data = data.clone();

                        smol::spawn(async move {
                            launcher_config::write(&data).await.unwrap();
                        })
                        .detach();
                    },
                )),
        )
        .padding(10.)
        .lens(AppState::config)
}

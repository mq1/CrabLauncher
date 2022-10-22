// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, Scroll, Switch, TextBox},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::launcher_config::{self, LauncherConfig},
    navbar, AppState,
};

pub fn build_widget() -> impl Widget<AppState> {
    let settings = Flex::column()
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
                            .with_child(Label::new("Automatically update JVM"))
                            .with_flex_spacer(1.)
                            .with_child(
                                Switch::new().lens(LauncherConfig::automatically_update_jvm),
                            )
                            .padding(5.)
                            .border(Color::GRAY, 1.)
                            .rounded(5.),
                    )
                    .with_default_spacer()
                    .with_child(
                        Flex::row()
                            .with_child(Label::new("Automatically optimize JVM arguments"))
                            .with_flex_spacer(1.)
                            .with_child(
                                Switch::new()
                                    .lens(LauncherConfig::automatically_optimize_jvm_arguments),
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
                .with_child(
                    Button::<LauncherConfig>::new("Reset to default settings 🔄").on_click(
                        |_, data, _| {
                            *data = LauncherConfig::default();
                        },
                    ),
                )
                .with_default_spacer()
                .with_child(Button::<LauncherConfig>::new("Save settings 📝").on_click(
                    |_, data, _| {
                        tokio::spawn(launcher_config::write(data.clone()));
                    },
                )),
        )
        .padding(10.)
        .lens(AppState::config);

    Flex::row()
        .with_child(navbar::build_widget())
        .with_flex_child(settings, 1.)
}

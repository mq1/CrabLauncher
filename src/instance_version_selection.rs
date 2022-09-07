// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    widget::{Button, Checkbox, CrossAxisAlignment, Either, Flex, Label, List, Scroll, Spinner},
    Color, UnitPoint, Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    let loading_versions = Flex::column()
        .with_child(Label::new("Fetching available versions..."))
        .with_default_spacer()
        .with_child(Spinner::new())
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER);

    let version_selector = Flex::row()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(
                            |(_, selected): &(_, bool), _env: &_| {
                                if *selected {
                                    "✅"
                                } else {
                                    "☑️"
                                }
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Label::new(|(version, _): &(String, _), _env: &_| {
                            version.to_owned()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("✅ Select").on_click(
                            |ctx, (version, _): &mut (String, _), _env: &_| {
                                let event_sink = ctx.get_external_handle();
                                select_version(event_sink, version);
                            },
                        ))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::version_selection),
            )
            .vertical(),
            1.,
        )
        .with_default_spacer()
        .with_child(
            Flex::column()
                .with_child(Label::new("🔎 Filter"))
                .with_default_spacer()
                .with_child(Checkbox::new("Show releases").lens(AppState::show_releases))
                .with_default_spacer()
                .with_child(Checkbox::new("Show snapshots").lens(AppState::show_snapshots))
                .with_default_spacer()
                .with_child(Checkbox::new("Show old alphas").lens(AppState::show_old_alphas))
                .with_default_spacer()
                .with_child(Checkbox::new("Show old betas").lens(AppState::show_old_betas))
                .on_click(|ctx, _, _| {
                    let event_sink = ctx.get_external_handle();
                    refresh_versions(event_sink);
                }),
        );

    let either = Either::new(
        |data, _env| data.available_minecraft_versions.is_empty(),
        loading_versions,
        version_selector,
    );

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("📦 Select the version").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(either, 1.)
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("< Select type 🛠️").on_click(|_, data: &mut AppState, _| {
                        data.current_view = View::InstanceTypeSelection;
                    }),
                )
                .with_flex_spacer(1.)
                .with_child(Button::new("Select name ✍️ >").on_click(
                    |_, data: &mut AppState, _| {
                        data.current_view = View::InstanceNameSelection;
                    },
                )),
        )
        .padding(10.)
}

pub fn refresh_versions(event_sink: druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        let versions: Vector<(String, bool)> = data
            .available_minecraft_versions
            .iter()
            .filter_map(|version| match version.version_type {
                lib::minecraft_version_manifest::Type::Release => {
                    if data.show_releases {
                        Some((version.id.to_owned(), version.id == data.selected_version))
                    } else {
                        None
                    }
                }
                lib::minecraft_version_manifest::Type::Snapshot => {
                    if data.show_snapshots {
                        Some((version.id.to_owned(), version.id == data.selected_version))
                    } else {
                        None
                    }
                }
                lib::minecraft_version_manifest::Type::OldAlpha => {
                    if data.show_old_alphas {
                        Some((version.id.to_owned(), version.id == data.selected_version))
                    } else {
                        None
                    }
                }
                lib::minecraft_version_manifest::Type::OldBeta => {
                    if data.show_old_betas {
                        Some((version.id.to_owned(), version.id == data.selected_version))
                    } else {
                        None
                    }
                }
            })
            .collect();

        data.version_selection = versions;
    });
}

fn select_version(event_sink: druid::ExtEventSink, version: &str) {
    let version = version.to_owned();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.selected_version = version.clone();

        data.version_selection = data
            .version_selection
            .iter()
            .map(|(v, _)| (v.to_owned(), v == &version))
            .collect();
    });
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    widget::{Button, Checkbox, CrossAxisAlignment, Flex, Label, RadioGroup, Scroll},
    LensExt, Widget, WidgetExt,
};

use crate::{
    lib::{self, minecraft_version_manifest::VersionType},
    AppState, NewInstanceState, View,
};

pub fn build_widget(
    available_versions: &Vector<lib::minecraft_version_manifest::Version>,
) -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("📦 Select the version").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Flex::row()
                .with_flex_child(
                    Scroll::new(
                        RadioGroup::column(
                            available_versions
                                .iter()
                                .map(|version| (version.id.clone(), Some(version.clone())))
                                .collect::<Vector<_>>(),
                        )
                        .expand_width()
                        .lens(
                            AppState::new_instance_state.then(NewInstanceState::selected_version),
                        ),
                    )
                    .vertical(),
                    1.,
                )
                .with_default_spacer()
                .with_child(
                    Flex::column()
                        .with_child(Label::new("🔎 Filter").with_text_size(32.))
                        .with_default_spacer()
                        .with_child(Checkbox::new("Show releases").lens(
                            AppState::new_instance_state.then(NewInstanceState::show_releases),
                        ))
                        .with_default_spacer()
                        .with_child(Checkbox::new("Show snapshots").lens(
                            AppState::new_instance_state.then(NewInstanceState::show_snapshots),
                        ))
                        .with_default_spacer()
                        .with_child(
                            Checkbox::new("Show old betas").lens(
                                AppState::new_instance_state.then(NewInstanceState::show_beta),
                            ),
                        )
                        .with_default_spacer()
                        .with_child(
                            Checkbox::new("Show old alphas").lens(
                                AppState::new_instance_state.then(NewInstanceState::show_alpha),
                            ),
                        ),
                )
                .on_click(|ctx, data, _| {
                    let event_sink = ctx.get_external_handle();
                    let new_instance_state = data.new_instance_state.clone();
                    data.current_view = View::LoadingVersions;

                    smol::spawn(async move {
                        refresh_shown_versions(event_sink, new_instance_state).await;
                    })
                    .detach();
                }),
            1.,
        )
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

pub async fn refresh_shown_versions(
    event_sink: druid::ExtEventSink,
    new_instance_state: NewInstanceState,
) {
    let shown_versions = new_instance_state
        .available_minecraft_versions
        .into_iter()
        .filter(|version| match version.version_type {
            VersionType::Release => new_instance_state.show_releases,
            VersionType::Snapshot => new_instance_state.show_snapshots,
            VersionType::OldBeta => new_instance_state.show_beta,
            VersionType::OldAlpha => new_instance_state.show_alpha,
        })
        .collect();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.new_instance_state.shown_minecraft_versions = shown_versions;
        data.current_view = View::InstanceVersionSelection;
    });
}

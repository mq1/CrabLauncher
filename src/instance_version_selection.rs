// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, RadioGroup, Scroll},
    LensExt, Widget, WidgetExt,
};

use crate::{lib::minecraft_version_manifest::Version, AppState, NewInstanceState, View};

pub fn build_widget(available_versions: &Vector<Version>) -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("📦 Select the version").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                RadioGroup::column(
                    available_versions
                        .iter()
                        .map(|version| (version.id.clone(), Some(version.clone())))
                        .collect::<Vector<_>>(),
                )
                .expand_width()
                .lens(AppState::new_instance_state.then(NewInstanceState::selected_version)),
            )
            .vertical(),
            1.,
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(
                    Button::<AppState>::new("< Select type 🛠️").on_click(|_, data, _| {
                        data.current_view = View::InstanceTypeSelection;
                    }),
                )
                .with_flex_spacer(1.)
                .with_child(
                    Button::<AppState>::new("Select name ✍️ >").on_click(|_, data, _| {
                        data.current_view = View::InstanceNameSelection;
                    }),
                ),
        )
        .padding(10.)
}

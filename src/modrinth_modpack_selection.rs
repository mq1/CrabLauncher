// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib::modrinth::Hit, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("📦 Modpack selection").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::<Hit>::dynamic(|data, _| data.title.to_owned()))
                        .with_flex_spacer(1.)
                        .with_child(Button::<Hit>::new("Select").on_click(|ctx, data, _| {
                            let hit = data.clone();
                            let event_sink = ctx.get_external_handle();
                            event_sink.add_idle_callback(move |data: &mut AppState| {
                                data.selected_modrinth_hit = Some(hit);
                                data.current_view = View::ModrinthModpack;
                            });
                        }))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::modrinth_hits),
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
                .with_flex_spacer(1.),
        )
        .padding(10.)
}

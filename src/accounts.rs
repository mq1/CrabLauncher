// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib::accounts::Account, AppState};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("👥 Accounts").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new("👤"))
                        .with_default_spacer()
                        .with_child(Label::new(|(_, account): &(_, Account), _env: &_| {
                            account.minecraft_username.to_string()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Select ✅"))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::accounts),
            )
            .vertical(),
            1.,
        )
        .padding(10.)
}

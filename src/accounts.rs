// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::accounts::{self, Account},
    AppState,
};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("👥 Accounts").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(
                            |(_, _, is_active): &(_, _, bool), _env: &_| {
                                if *is_active {
                                    "✅"
                                } else {
                                    "☑️"
                                }
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Label::new(|(_, account, _): &(_, Account, _), _env: &_| {
                            account.minecraft_username.to_string()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Select ✅").on_click(
                            |ctx, (id, _, _): &mut (String, _, _), _env| {
                                accounts::set_active(id).expect("Failed to set active account");
                                let event_sink = ctx.get_external_handle();
                                update_accounts(event_sink);
                            },
                        ))
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

pub fn update_accounts(event_sink: druid::ExtEventSink) {
    let accounts = accounts::list().unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| data.accounts = accounts);
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::{accounts, msa::AccountEntry, self},
    AppState,
};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("👥 Accounts").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(
                            |(_, is_active): &(_, bool), _env: &_| {
                                if *is_active {
                                    "✅"
                                } else {
                                    "☑️"
                                }
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Label::new(|(entry, _): &(AccountEntry, _), _env: &_| {
                            entry.account.minecraft_username.to_string()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Remove 💣").on_click(
                            |ctx, (entry, _): &mut (AccountEntry, _), _| {
                                accounts::remove(&entry.minecraft_id).expect("Failed to remove account");
                                let event_sink = ctx.get_external_handle();
                                update_accounts(event_sink);
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Button::new("Select ✅").on_click(
                            |ctx, (entry, _): &mut (AccountEntry, _), _env| {
                                accounts::set_active(&entry.minecraft_id).expect("Failed to set active account");
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
        )
        .with_default_spacer()
        .with_child(
            Button::new("New Account 🎉").on_click(|ctx, _, _| {
                let event_sink = ctx.get_external_handle();
                add_account(event_sink);
            }),
        )
        .padding(10.)
}

fn update_accounts(event_sink: druid::ExtEventSink) {
    let accounts = accounts::list().unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| data.accounts = accounts);
}

fn add_account(event_sink: druid::ExtEventSink) {
    open::that(lib::msa::AUTH_URL.as_str()).expect("Failed to open auth url");
    lib::accounts::add().expect("Failed to add account");
    update_accounts(event_sink);
}

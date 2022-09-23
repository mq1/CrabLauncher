// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib, AppState};

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
                            |account: &lib::msa::Account, _env: &_| {
                                if account.is_active {
                                    "✅"
                                } else {
                                    "☑️"
                                }
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Label::new(|account: &lib::msa::Account, _env: &_| {
                            account.mc_username.to_string()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Remove 💣").on_click(
                            |ctx, account: &mut lib::msa::Account, _| {
                                let event_sink = ctx.get_external_handle();
                                smol::spawn(remove_account(event_sink, account.clone())).detach();
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Button::new("Select ✅").on_click(
                            |ctx, account: &mut lib::msa::Account, _env| {
                                let event_sink = ctx.get_external_handle();

                                smol::spawn(set_active_account(event_sink, account.clone()))
                                    .detach();
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
        .with_child(Button::new("New Account 🎉").on_click(|ctx, _, _| {
            let event_sink = ctx.get_external_handle();
            smol::spawn(add_account(event_sink)).detach();
        }))
        .with_flex_spacer(1.)
        .padding(10.)
}

async fn remove_account(event_sink: druid::ExtEventSink, account: lib::msa::Account) {
    let id = account.mc_id.clone();

    smol::spawn(lib::accounts::remove(account)).detach();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.accounts.retain(|a| a.mc_id != id);
    });
}

async fn set_active_account(event_sink: druid::ExtEventSink, account: lib::msa::Account) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.accounts.iter_mut().for_each(|a| {
            a.is_active = a.mc_id == account.mc_id;
        });

        let accounts = data.accounts.clone().into_iter().collect();
        smol::spawn(lib::accounts::update_accounts(accounts)).detach();
    });
}

async fn add_account(event_sink: druid::ExtEventSink) {
    open::that(lib::msa::AUTH_URL.as_str()).expect("Failed to open auth url");
    let account = lib::accounts::add().await.expect("Failed to add account");

    event_sink.add_idle_callback(|data: &mut AppState| {
        data.accounts.push_front(account);
    });
}

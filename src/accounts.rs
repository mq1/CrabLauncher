// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    lens,
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, LensExt, Widget, WidgetExt,
};
use oauth2::PkceCodeVerifier;

use crate::{
    lib::{self, msa::Account},
    AppState, View,
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
                        .with_child(Label::<(_, Account)>::dynamic(
                            |(_, account), _| match account.is_active {
                                true => "✅".to_string(),
                                false => "☑️".to_string(),
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Label::<(_, Account)>::dynamic(|(_, account), _| {
                            account.mc_username.to_owned()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(
                            Button::<(Vector<Account>, Account)>::new("Remove 💣").on_click(
                                |_, (accounts, account), _| {
                                    tokio::spawn(lib::accounts::remove(account.clone()));
                                    accounts.retain(|a| a.mc_id != account.mc_id);
                                },
                            ),
                        )
                        .with_default_spacer()
                        .with_child(
                            Button::<(Vector<Account>, Account)>::new("Select ✅").on_click(
                                |_, (accounts, account), _| {
                                    tokio::spawn(lib::accounts::set_active(account.clone()));

                                    accounts.iter_mut().for_each(|a| {
                                        a.is_active = a.mc_id == account.mc_id;
                                    });
                                },
                            ),
                        )
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(lens::Identity.map(
                    |data: &AppState| (data.accounts.clone(), data.accounts.clone()),
                    |data: &mut AppState, (accounts, _)| {
                        data.accounts = accounts;
                    },
                )),
            )
            .vertical(),
        )
        .with_default_spacer()
        .with_child(
            Button::<AppState>::new("New Account 🎉").on_click(|ctx, data, _| {
                data.loading_message = "Waiting for authentication...".to_string();
                data.current_view = View::Loading;

                let (auth_url, pkce_verifier) = lib::msa::get_auth_url();
                open::that(auth_url.to_string()).expect("Failed to open auth url");

                let event_sink = ctx.get_external_handle();
                tokio::spawn(add_account(event_sink, pkce_verifier));
            }),
        )
        .with_flex_spacer(1.)
        .padding(10.)
}

async fn add_account(event_sink: druid::ExtEventSink, pkce_verifier: PkceCodeVerifier) {
    lib::accounts::add(pkce_verifier).await.unwrap();
    let accounts = lib::accounts::read().await.unwrap().accounts;

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.accounts = accounts;
        data.current_view = View::Accounts;
    });
}

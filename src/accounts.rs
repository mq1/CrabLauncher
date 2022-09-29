// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("👥 Accounts").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::dynamic(get_emoji))
                        .with_default_spacer()
                        .with_child(Label::dynamic(get_name))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Remove 💣").on_click(remove_account))
                        .with_default_spacer()
                        .with_child(Button::new("Select ✅").on_click(select_account))
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
        .with_child(Button::new("New Account 🎉").on_click(add_account))
        .with_flex_spacer(1.)
        .padding(10.)
}

fn get_emoji(account: &lib::msa::Account, _: &druid::Env) -> String {
    match account.is_active {
        true => "✅".to_string(),
        false => "☑️".to_string(),
    }
}

fn get_name(account: &lib::msa::Account, _: &druid::Env) -> String {
    account.mc_username.to_string()
}

fn remove_account(ctx: &mut druid::EventCtx, account: &mut lib::msa::Account, _: &druid::Env) {
    smol::spawn(lib::accounts::remove(account.clone())).detach();

    let event_sink = ctx.get_external_handle();
    let mc_id = account.mc_id.clone();
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.accounts.retain(|a| a.mc_id != mc_id);
    });
}

fn select_account(ctx: &mut druid::EventCtx, account: &mut lib::msa::Account, _: &druid::Env) {
    smol::spawn(lib::accounts::set_active(account.clone())).detach();

    let event_sink = ctx.get_external_handle();
    let mc_id = account.mc_id.clone();
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.accounts.iter_mut().for_each(|a| {
            a.is_active = a.mc_id == mc_id;
        });
    });
}

fn add_account(ctx: &mut druid::EventCtx, data: &mut AppState, _: &druid::Env) {
    data.loading_message = "Waiting for authentication...".to_string();
    data.current_view = View::Loading;
    open::that(lib::msa::AUTH_URL.as_str()).expect("Failed to open auth url");

    let event_sink = ctx.get_external_handle();
    smol::spawn(login(event_sink)).detach();
}

async fn login(event_sink: druid::ExtEventSink) {
    let account = lib::accounts::add().await.expect("Failed to add account");

    event_sink.add_idle_callback(|data: &mut AppState| {
        data.accounts.push_front(account);
        data.current_view = View::Accounts;
    });
}

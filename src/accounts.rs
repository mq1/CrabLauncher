// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::{self, msa::Account},
    navbar, AppState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    let accounts = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("👥 Accounts").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::<Account>::dynamic(|account, _| {
                            match account.is_active {
                                true => "✅".to_string(),
                                false => "☑️".to_string(),
                            }
                        }))
                        .with_default_spacer()
                        .with_child(Label::<Account>::dynamic(|account, _| {
                            account.mc_username.to_owned()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(Button::<Account>::new("Remove 💣").on_click(
                            |ctx, account, _| {
                                let event_sink = ctx.get_external_handle();
                                let account = account.to_owned();
                                event_sink.add_idle_callback(move |data: &mut AppState| {
                                    data.selected_account = Some(account);
                                    data.current_view = View::ConfirmAccountRemove;
                                });
                            },
                        ))
                        .with_default_spacer()
                        .with_child(Button::<Account>::new("Select ✅").on_click(
                            |ctx, account, _| {
                                let account = account.to_owned();
                                let event_sink = ctx.get_external_handle();
                                lib::accounts::set_active(account, event_sink).unwrap();
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
            Button::<AppState>::new("New Account 🎉").on_click(|ctx, _, _| {
                let event_sink = ctx.get_external_handle();
                thread::spawn(move || lib::accounts::add(event_sink));
            }),
        )
        .with_flex_spacer(1.)
        .padding(10.);

    Flex::row()
        .with_child(navbar::build_widget())
        .with_flex_child(accounts, 1.)
}

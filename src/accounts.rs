// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    lens,
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, LensExt, Widget, WidgetExt,
};

use crate::{
    lib::{self, msa::Account},
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
                                |ctx, (accounts, account), _| {
                                    accounts.iter_mut().for_each(|a| {
                                        a.is_active = a.mc_id == account.mc_id;
                                    });

                                    let event_sink = ctx.get_external_handle();
                                    tokio::spawn(lib::accounts::set_active(
                                        account.clone(),
                                        event_sink,
                                    ));
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
            Button::<AppState>::new("New Account 🎉").on_click(|ctx, _, _| {
                let event_sink = ctx.get_external_handle();
                tokio::spawn(lib::accounts::add(event_sink));
            }),
        )
        .with_flex_spacer(1.)
        .padding(10.)
}

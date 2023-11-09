// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use crate::components::adding_account;
use crate::types::accounts::Accounts;
use eframe::egui;
use poll_promise::Promise;
use std::thread;

pub fn show(ctx: &egui::Context, app: &mut App) {
    let adding_account_modal = adding_account::modal(ctx, app);

    if let Some(promise) = &mut app.adding_account {
        if let Some(account) = promise.ready() {
            app.accounts.add_account(account.clone()).unwrap();
            app.adding_account = None;
        } else {
            adding_account_modal.open();
        }
    }

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            if ui.button("➕ Add Account").clicked() {
                let client = Accounts::get_client().unwrap();
                let details = Accounts::get_details(&client).unwrap();

                app.adding_account_details = (
                    details.verification_uri().to_string(),
                    details.user_code().secret().to_string(),
                );

                let (sender, promise) = Promise::new();
                thread::spawn(move || {
                    let account = Accounts::get_account(client, details).unwrap();
                    sender.send(account);
                });

                app.adding_account = Some(promise);
            }
        });
        ui.add_space(4.);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Accounts");

        ui.separator();
        ui.add_space(16.);

        egui::Grid::new("accounts").striped(true).show(ui, |ui| {
            for account in app.accounts.accounts.clone() {
                if ui
                    .add(egui::RadioButton::new(
                        app.accounts.active_mc_id == account.mc_id,
                        account.mc_username.clone(),
                    ))
                    .clicked()
                {
                    app.accounts.set_active_account(&account.mc_id).unwrap();
                }

                ui.end_row();
            }
        });
    });
}

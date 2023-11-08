// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use eframe::egui;

pub fn show(ctx: &egui::Context, app: &mut App) {
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

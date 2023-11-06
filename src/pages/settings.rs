// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use crate::components::error;
use eframe::egui;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            if ui.button("💾 Save").clicked() {
                if let Err(error) = app.settings.save() {
                    error::show(ctx, &error);
                }
            }
        });
        ui.add_space(4.);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");
        ui.separator();
    });
}

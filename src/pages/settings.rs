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
        ui.add_space(16.);

        ui.group(|ui| {
            ui.heading("Launcher");
            ui.separator();

            egui::Grid::new("launcher_settings")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Check for updates");
                    ui.checkbox(&mut app.settings.check_for_updates, "");
                    ui.end_row();
                });
        });

        ui.add_space(16.);

        ui.group(|ui| {
            ui.heading("Java");
            ui.separator();

            egui::Grid::new("java_settings")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Java path");
                    ui.text_edit_singleline(&mut app.settings.java_path);
                    ui.end_row();

                    ui.label("Java memory");
                    ui.text_edit_singleline(&mut app.settings.java_memory);
                    ui.end_row();
                });
        });
    });
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use eframe::egui;

pub fn show(ctx: &egui::Context, app: &mut App) {
    if app.vanilla_installer.versions.is_none() {
        app.vanilla_installer.fetch_versions();
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Vanilla Installer");

        ui.separator();
        ui.add_space(16.);

        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut app.vanilla_installer.name);
            });

            ui.add_space(8.);

            ui.group(|ui| {
                ui.label("Version");
                egui::ComboBox::from_id_source("version_selector")
                    .selected_text(&app.vanilla_installer.selected_version)
                    .show_ui(ui, move |ui| {
                        if let Some(versions) = &app.vanilla_installer.versions {
                            if let Some(versions) = versions.ready() {
                                for version in versions {
                                    ui.selectable_value(
                                        &mut app.vanilla_installer.selected_version,
                                        version.clone(),
                                        version,
                                    );
                                }
                            }
                        }
                    });
            });
        });
    });
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use eframe::egui;
use egui_modal::Modal;

pub fn vanilla_installer(ctx: &egui::Context, app: &mut App) -> Modal {
    let modal = Modal::new(ctx, "vanilla_installer_modal");
    modal.show(|ui| {
        modal.frame(ui, |ui| {
            ui.set_min_size(egui::Vec2::new(512., 256.));

            ui.heading("Vanilla Installer");
            ui.separator();
            ui.add_space(16.);

            ui.vertical(|ui| {
                ui.heading("Name");
                ui.text_edit_singleline(&mut app.vanilla_installer.name);

                ui.add_space(8.);

                ui.heading("Version");
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
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });

    modal
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;

use crate::types::vanilla_installer::VanillaInstaller;

pub fn view(ctx: &egui::Context, vanilla_installer: &VanillaInstaller, selected_version: &mut String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Name");

        ui.separator();

        ui.heading("Version");

        egui::ComboBox::from_id_source("version_selector").selected_text(selected_version.as_str()).show_ui(ui, move |ui| {
            if let Some(versions) = &vanilla_installer.versions {
                if let Some(versions) = versions.ready() {
                    for version in versions {
                        ui.selectable_value(selected_version, version.clone(), version);
                    }
                }
            }
        });
    });
}

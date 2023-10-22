// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use crate::components::vanilla_installer::vanilla_installer;
use crate::types::instances::INSTANCES_DIR;
use eframe::egui;

pub fn footer(ctx: &egui::Context, app: &mut App) {
    let vanilla_installer = vanilla_installer(ctx, app);

    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            if ui.button("📂 Open instances folder").clicked() {
                open::that(&*INSTANCES_DIR).unwrap();
            }
            if ui.button("✨ New Vanilla Instance").clicked() {
                if app.vanilla_installer.versions.is_none() {
                    app.vanilla_installer.fetch_versions();
                }

                vanilla_installer.open();
            }
        });
        ui.add_space(4.);
    });
}

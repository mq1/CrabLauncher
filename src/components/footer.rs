// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use crate::types::instances::INSTANCES_DIR;
use eframe::egui;

pub fn footer(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            if ui.button("📂 Open instances folder").clicked() {
                open::that(&*INSTANCES_DIR).unwrap();
            }
        });
        ui.add_space(4.);
    });
}

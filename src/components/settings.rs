// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;
use egui_modal::Modal;

pub fn settings(ctx: &egui::Context) -> Modal {
    let modal = Modal::new(ctx, "settings_modal");
    modal.show(|ui| {
        modal.frame(ui, |ui| {
            ui.heading("Settings");
            ui.add_space(8.);
            ui.label("Nothing to see here yet");
        });
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });

    modal
}

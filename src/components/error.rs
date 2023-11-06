// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use eframe::egui;
use egui_modal::{Icon, Modal};

pub fn show(ctx: &egui::Context, error: &anyhow::Error) {
    let modal = Modal::new(ctx, "error_modal");
    modal.show(|ui| {
        modal.title(ui, "Error");
        modal.frame(ui, |ui| {
            modal.body_and_icon(ui, error.to_string(), Icon::Error)
        });
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });
}

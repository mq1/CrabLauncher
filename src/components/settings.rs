// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;
use egui_modal::{Modal, ModalStyle};

pub fn settings_modal(ctx: &egui::Context) -> Modal {
    let style = ModalStyle {
        body_alignment: egui::Align::Min,
        ..Default::default()
    };

    let modal = Modal::new(ctx, "settings_modal").with_style(&style);
    modal.show(|ui| {
        modal.title(ui, "Settings");
        modal.frame(ui, |ui| {
            modal.body(ui, "Nothing here yet");
        });
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });

    modal
}

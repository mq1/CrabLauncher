// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;
use egui_modal::{Modal, ModalStyle};

pub fn settings_modal(ctx: &egui::Context) -> Modal {
    let window_size = ctx.available_rect().size();

    let style = ModalStyle {
        body_alignment: egui::Align::Min,
        default_height: Some(window_size.y * 0.75),
        default_width: Some(window_size.x * 0.75),
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

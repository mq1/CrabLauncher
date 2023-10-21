// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;
use egui_modal::Modal;

const APP_VERSION: &str = concat!("CrabLauncher v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = concat!(env!("CARGO_PKG_LICENSE"), " Licensed");
const COPYRIGHT: &str = concat!("Copyright © 2023 ", env!("CARGO_PKG_AUTHORS"));
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn about(ctx: &egui::Context) -> Modal {
    let modal = Modal::new(ctx, "about_modal");
    modal.show(|ui| {
        modal.frame(ui, |ui| {
            ui.heading(APP_VERSION);
            ui.add_space(16.);
            ui.label(LICENSE);
            ui.add_space(8.);
            ui.label(COPYRIGHT);
            ui.add_space(8.);
            ui.hyperlink(REPOSITORY);
        });
        modal.buttons(ui, |ui| {
            modal.button(ui, "Close");
        });
    });

    modal
}

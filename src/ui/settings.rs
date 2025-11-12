// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use eframe::egui;

pub fn update(ctx: &egui::Context, _frame: &mut eframe::Frame, _app: &mut App) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");
    });
}

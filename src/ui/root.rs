// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{app::App, ui};
use eframe::egui;

pub fn update(ctx: &egui::Context, frame: &mut eframe::Frame, app: &mut App) {
    ui::nav::update(ctx, frame, app);

    match app.current_view {
        ui::View::Instances => ui::instances::update(ctx, frame, app),
        ui::View::Settings => ui::settings::update(ctx, frame, app),
    }

    match app.modal {
        ui::Modal::None => {}
        ui::Modal::Info => ui::info::update(ctx, app),
    }
}

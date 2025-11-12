// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{app::App, ui};
use eframe::egui::{self, Vec2};

pub fn update(ctx: &egui::Context, _frame: &mut eframe::Frame, app: &mut App) {
    let frame =
        egui::Frame::side_top_panel(&ctx.style()).fill(ctx.style().visuals.extreme_bg_color);

    egui::SidePanel::left("nav")
        .resizable(false)
        .exact_width(57.)
        .frame(frame)
        .show(ctx, |ui| {
            ui.add_space(6.);

            if ui
                .add_sized(
                    Vec2::splat(40.),
                    egui::Button::selectable(
                        app.current_view == ui::View::Instances,
                        egui::RichText::new("🔲").size(26.),
                    ),
                )
                .on_hover_text("View your Minecraft instances")
                .clicked()
            {
                app.change_view(ctx, ui::View::Instances);
            }

            if ui
                .add_sized(
                    Vec2::splat(40.),
                    egui::Button::selectable(
                        app.current_view == ui::View::Settings,
                        egui::RichText::new("⛭").size(26.),
                    ),
                )
                .on_hover_text("Settings")
                .clicked()
            {
                app.change_view(ctx, ui::View::Settings);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(6.);

                if ui
                    .add_sized(
                        Vec2::splat(40.),
                        egui::Button::new(egui::RichText::new("ℹ").size(26.)),
                    )
                    .on_hover_text(format!("{} Info", env!("CARGO_PKG_NAME")))
                    .clicked()
                {
                    app.open_modal(ui::Modal::Info);
                }
            });
        });
}

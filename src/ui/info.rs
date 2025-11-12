// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{LOGO_BYTES, app::App};
use eframe::egui;

pub fn update(ctx: &egui::Context, app: &mut App) {
    let modal = egui::Modal::new("info".into());

    modal.show(ctx, |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            ui.set_height(68.);

            ui.add(
                egui::Image::from_bytes("bytes://info", LOGO_BYTES)
                    .max_size(egui::Vec2::splat(64.)),
            );

            ui.vertical(|ui| {
                ui.add_space(4.);
                ui.heading(env!("CARGO_PKG_NAME"));
                ui.label(format!("📌 Version {}", env!("CARGO_PKG_VERSION")));
                ui.label("© 2025 Manuel Quarneti | 📃 GPL-3.0-only");
            });
        });

        ui.separator();

        ui.add_space(10.);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
            if ui.button("❌ Close").clicked() {
                app.close_modal();
            }

            ui.add_sized(egui::Vec2::new(1., 21.), egui::Separator::default());

            if ui.button("📁 Open Data Directory").clicked()
                && let Err(e) = app.open_data_dir()
            {
                app.notifications.show_err(e.into());
            }

            if ui.button(" Source Code").clicked()
                && let Err(e) = open::that(env!("CARGO_PKG_REPOSITORY"))
            {
                app.notifications.show_err(e.into());
            }
        })
    });
}

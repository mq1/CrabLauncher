// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use crate::types::instances::INSTANCES_DIR;
use eframe::egui;
use egui_modal::Modal;

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.add_space(8.);
        ui.horizontal(|ui| {
            if ui.button("📂 Open instances folder").clicked() {
                open::that(&*INSTANCES_DIR).unwrap();
            }
        });
        ui.add_space(4.);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Instances");
        ui.separator();

        for instance in &app.instances.list {
            ui.group(|ui| {
                ui.set_max_width(128.);

                let img = egui::include_image!("../../assets/grass-128x128.png");
                let img = egui::Image::new(img).max_width(64.).max_height(64.);

                ui.add(img);
                ui.label(&instance.name);

                ui.separator();

                ui.button("▶ Play").clicked();
                ui.button("⚙ Settings").clicked();

                let modal = Modal::new(ctx, "delete_instance_modal");
                modal.show(|ui| {
                    modal.frame(ui, |ui| {
                        ui.heading("Delete instance");
                        ui.add_space(8.);
                        ui.label("Are you sure you want to delete this instance?");
                    });
                    modal.buttons(ui, |ui| {
                        if ui.button("Cancel").clicked() {
                            modal.close();
                        }
                        if ui.button("🗑 Delete").clicked() {
                            instance.delete().unwrap();
                            modal.close();
                        }
                    });
                });
                if ui.button("🗑 Delete").clicked() {
                    modal.open();
                }

                if ui.button("📂 Open folder").clicked() {
                    instance.open_dir().unwrap();
                }
            });
        }
    });
}

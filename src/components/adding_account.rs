// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::app::App;
use eframe::egui;
use egui_modal::Modal;

pub fn modal(ctx: &egui::Context, app: &App) -> Modal {
    let modal = Modal::new(ctx, "adding_account");
    modal.show(|ui| {
        modal.title(ui, "Adding Account");
        modal.frame(ui, |ui| {
            modal.body(ui, "Please go to the following URL and enter the code:");
            ui.add_space(8.);
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.add_space(8.);
                ui.hyperlink(&app.adding_account_details.0);
            });
            ui.add_space(8.);
            ui.horizontal(|ui| {
                ui.label("Code:");
                ui.add_space(8.);
                ui.label(&app.adding_account_details.1);
            });
        });
        modal.buttons(ui, |ui| {
            modal.button(ui, "Cancel");
            if ui.button("Copy Code and open URL").clicked() {
                ctx.output_mut(|o| o.copied_text = app.adding_account_details.1.clone());
                ctx.output_mut(|o| {
                    o.open_url = Some(egui::OpenUrl {
                        url: app.adding_account_details.0.clone(),
                        new_tab: true,
                    })
                });

                modal.close();
            }
        });
    });

    modal
}

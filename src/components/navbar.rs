// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::components::about::about;
use crate::components::navigation_button::navigation_button;
use crate::components::settings::settings;
use eframe::egui;

pub fn navbar(ctx: &egui::Context) {
    // disable panel separators
    ctx.style_mut(|style| {
        style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::TRANSPARENT;
    });

    let about_modal = about(ctx);
    let settings_modal = settings(ctx);

    let mut navbar_frame = egui::Frame::none();
    navbar_frame.fill = egui::Color32::from_rgb(23, 23, 23);

    egui::TopBottomPanel::top("navbar")
        .exact_height(48.)
        .frame(navbar_frame)
        .show(ctx, |ui| {
            // disable item spacing
            ui.spacing_mut().item_spacing = egui::Vec2::new(0., 0.);

            // set frame borders to zero
            ui.style_mut().visuals.widgets.hovered.bg_stroke.width = 0.;
            ui.style_mut().visuals.widgets.active.bg_stroke.width = 0.;

            // set button padding
            ui.style_mut().spacing.button_padding = egui::Vec2::new(12., 12.);

            // set button border radius
            ui.style_mut().visuals.widgets.active.rounding = egui::Rounding::ZERO;
            ui.style_mut().visuals.widgets.hovered.rounding = egui::Rounding::ZERO;

            // set button background color
            ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(23, 23, 23);
            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(23, 23, 23);

            ui.horizontal_centered(|ui| {
                ui.add_space(8.);

                ui.heading("Instances");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui
                        .add(navigation_button(egui::include_image!(
                            "../../assets/mdi/information-outline.svg"
                        )))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        about_modal.open();
                    }

                    if ui
                        .add(navigation_button(egui::include_image!(
                            "../../assets/mdi/cog-outline.svg"
                        )))
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        settings_modal.open();
                    }
                });
            });
        });

    // re-enable separators
    ctx.style_mut(|style| {
        style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::DARK_GRAY;
    });
}

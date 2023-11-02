// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use crate::components::navigation_button;
use crate::pages::Page;
use eframe::egui;

pub fn navbar(ctx: &egui::Context, app: &mut App) {
    // disable panel separators
    ctx.style_mut(|style| {
        style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::TRANSPARENT;
    });

    let mut navbar_frame = egui::Frame::none();
    navbar_frame.fill = egui::Color32::from_rgb(23, 23, 23);

    egui::SidePanel::left("navbar")
        .exact_width(48.)
        .frame(navbar_frame)
        .resizable(false)
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

            navigation_button::show(
                ui,
                app,
                egui::include_image!("../../assets/mdi/view-grid-outline.svg"),
                Page::Instances,
            );

            navigation_button::show(
                ui,
                app,
                egui::include_image!("../../assets/mdi/view-grid-plus-outline.svg"),
                Page::VanillaInstaller,
            );

            navigation_button::show(
                ui,
                app,
                egui::include_image!("../../assets/mdi/cog-outline.svg"),
                Page::Settings,
            );
        });

    // re-enable separators
    ctx.style_mut(|style| {
        style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::DARK_GRAY;
    });
}

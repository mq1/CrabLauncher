// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::app::App;
use crate::pages::Page;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut App, image_source: egui::ImageSource, target: Page) {
    let img = egui::Image::new(image_source).fit_to_exact_size(egui::vec2(24., 24.));

    let color = if app.page == target {
        egui::Color32::from_rgb(50, 50, 50)
    } else {
        egui::Color32::TRANSPARENT
    };

    let btn = egui::Button::image(img)
        .min_size(egui::vec2(48., 48.))
        .fill(color);

    if ui
        .add(btn)
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
    {
        app.page = target;
    }
}

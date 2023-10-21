// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;

pub fn navigation_button(image_source: egui::ImageSource) -> egui::Button {
    let img = egui::Image::new(image_source).fit_to_exact_size(egui::vec2(24., 24.));

    let btn = egui::Button::image(img)
        .min_size(egui::vec2(48., 48.))
        .fill(egui::Color32::TRANSPARENT);

    btn
}

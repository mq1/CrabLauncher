// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuel.quarneti@proton.me>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;

pub fn navigation_button(image_source: egui::ImageSource, is_selected: bool) -> egui::Button {
    let img = egui::Image::new(image_source).max_width(32.).max_height(32.);
    let btn = egui::Button::image(img).fill(if is_selected { egui::Color32::from_rgb(0x2d, 0x2d, 0x2d) } else { egui::Color32::TRANSPARENT });

    btn
}

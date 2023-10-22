// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::components::footer::footer;
use crate::components::instances::instances;
use crate::components::navbar::navbar;
use eframe::egui;

use crate::types::instances::Instances;
use crate::types::vanilla_installer::VanillaInstaller;

pub struct App {
    pub instances: Instances,
    pub vanilla_installer: VanillaInstaller,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx.style_mut(|style| {
            style.visuals = egui::Visuals::dark();
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(0xd3, 0xd3, 0xd3));
        });

        Self {
            instances: Instances::new().unwrap(),
            vanilla_installer: VanillaInstaller::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        navbar(ctx);
        footer(ctx, self);
        instances(ctx, self);
    }
}

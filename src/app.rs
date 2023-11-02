// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::components::footer::footer;
use crate::components::navbar::navbar;
use crate::pages;
use crate::pages::Page;
use eframe::egui;

use crate::types::instances::Instances;
use crate::types::vanilla_installer::VanillaInstaller;

pub struct App {
    pub page: Page,
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
            page: Page::Instances,
            instances: Instances::new().unwrap(),
            vanilla_installer: VanillaInstaller::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        navbar(ctx, self);

        match self.page {
            Page::Instances => {
                footer(ctx, self);
                pages::instances::show(ctx, self);
            }
            Page::Settings => pages::settings::show(ctx, self),
            Page::VanillaInstaller => pages::vanilla_installer::show(ctx, self),
        }
    }
}

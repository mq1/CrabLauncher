// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use eframe::egui;

use crate::components::navbar;
use crate::pages;
use crate::pages::Page;
use crate::types::instances::Instances;
use crate::types::settings::Settings;
use crate::types::vanilla_installer::VanillaInstaller;

pub struct App {
    pub page: Page,
    pub instances: Instances,
    pub settings: Settings,
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
            settings: Settings::load().unwrap(),
            vanilla_installer: VanillaInstaller::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        navbar::show(ctx, self);

        match self.page {
            Page::Instances => pages::instances::show(ctx, self),
            Page::Settings => pages::settings::show(ctx, self),
            Page::VanillaInstaller => pages::vanilla_installer::show(ctx, self),
            Page::Info => pages::info::show(ctx, self),
        }
    }
}
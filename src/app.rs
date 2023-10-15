// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuel.quarneti@proton.me>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui;

use crate::components::navigation_button::navigation_button;
use crate::pages;
use crate::pages::Page;
use crate::types::instances::Instances;
use crate::types::vanilla_installer::VanillaInstaller;

pub struct App {
    page: Page,
    instances: Instances,
    vanilla_installer: VanillaInstaller,
    selected_version: String,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx.style_mut(|style| {
            // set dark theme
            style.visuals = egui::Visuals::dark();

            // set text color to light gray
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(0xd3, 0xd3, 0xd3));
        });

        Self {
            page: Page::Instances,
            instances: Instances::new().unwrap(),
            vanilla_installer: VanillaInstaller::new(),
            selected_version: String::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // disable panel separators
        ctx.style_mut(|style| {
            style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::TRANSPARENT;
        });

        let mut side_panel_frame = egui::Frame::none();
        side_panel_frame.fill = egui::Color32::from_rgb(23, 23, 23);

        egui::SidePanel::left("navigation_panel").exact_width(48.).resizable(false).frame(side_panel_frame).show(ctx, |ui| {
            // disable item spacing
            ui.spacing_mut().item_spacing = egui::Vec2::new(0., 0.);

            // set frame borders to zero
            ui.style_mut().visuals.widgets.hovered.bg_stroke.width = 0.;
            ui.style_mut().visuals.widgets.active.bg_stroke.width = 0.;

            // set button padding
            ui.style_mut().spacing.button_padding = egui::Vec2::new(8., 8.);

            if ui.add(navigation_button(
                egui::include_image!("../assets/mdi/view-grid-outline.svg"),
                self.page == Page::Instances,
            )).clicked() {
                self.page = Page::Instances;
            }

            if ui.add(navigation_button(
                egui::include_image!("../assets/mdi/view-grid-plus-outline.svg"),
                self.page == Page::NewInstance,
            )).clicked() {
                self.page = Page::NewInstance;
                self.vanilla_installer.fetch_versions();
            }

            if ui.add(navigation_button(
                egui::include_image!("../assets/mdi/information-outline.svg"),
                self.page == Page::About,
            )).clicked() {
                self.page = Page::About;
            }
        });

        // re-enable separators
        ctx.style_mut(|style| {
            style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::DARK_GRAY;
        });

        match self.page {
            Page::Instances => pages::instances::view(ctx, &self.instances),
            Page::NewInstance => pages::new_instance::view(ctx, &self.vanilla_installer, &mut self.selected_version),
            Page::About => pages::about::view(ctx),
        }
    }
}

// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod data_dir;
mod ui;

use crate::app::App;
use anyhow::Result;
use eframe::{
    NativeOptions,
    egui::{CornerRadius, ViewportBuilder, vec2},
};
use egui_extras::install_image_loaders;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const LOGO_BYTES: &[u8] = include_bytes!("../assets/CrabLauncher.png");

fn main() -> Result<()> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let data_dir = data_dir::get_data_dir()?;
    let app = App::new(data_dir);

    // Initialize UI

    let icon = if cfg!(target_os = "macos") {
        eframe::egui::IconData::default()
    } else {
        eframe::icon_data::from_png_bytes(LOGO_BYTES).expect("Failed to load icon")
    };

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800., 600.])
            .with_min_inner_size([745., 390.])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            //cc.egui_ctx.set_theme(app.config.contents.theme_preference);

            cc.egui_ctx.all_styles_mut(|style| {
                style.visuals.selection.bg_fill = ui::accent::get_accent_color();
                style.visuals.selection.stroke.color = style.visuals.strong_text_color();

                style.visuals.widgets.active.corner_radius = CornerRadius::same(30);
                style.visuals.widgets.hovered.corner_radius = CornerRadius::same(30);
                style.visuals.widgets.inactive.corner_radius = CornerRadius::same(30);
                style.visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
                style.visuals.widgets.open.corner_radius = CornerRadius::same(30);

                style.spacing.button_padding = vec2(5., 2.5);
            });

            Ok(Box::new(app))
        }),
    )
    .expect("Failed to run app");

    Ok(())
}

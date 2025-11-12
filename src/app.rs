// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use eframe::egui;

pub struct App {
    data_dir: PathBuf,
}

impl App {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }
}

impl eframe::App for App {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.heading("CrabLauncher");
        });
    }
}

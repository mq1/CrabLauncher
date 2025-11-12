// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{
    instances::{self, Instance},
    notifications::Notifications,
    ui,
};
use eframe::egui;
use std::{io, path::PathBuf};

pub struct App {
    data_dir: PathBuf,
    pub current_view: ui::View,
    instances: Box<[Instance]>,
    pub modal: ui::Modal,
    pub notifications: Notifications,
}

impl App {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            current_view: ui::View::Instances,
            instances: Box::new([]),
            modal: ui::Modal::None,
            notifications: Notifications::new(),
        }
    }

    pub fn refresh_instances(&mut self, _ctx: &egui::Context) {
        self.instances = instances::list(&self.data_dir);
    }

    pub fn change_view(&mut self, ctx: &egui::Context, view: ui::View) {
        self.current_view = view;

        let title = match self.current_view {
            ui::View::Instances => concat!(env!("CARGO_PKG_NAME"), " • Instances"),
            ui::View::Settings => concat!(env!("CARGO_PKG_NAME"), " • Settings"),
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title.to_string()));
    }

    pub fn open_modal(&mut self, modal: ui::Modal) {
        self.modal = modal;
    }

    pub fn close_modal(&mut self) {
        self.modal = ui::Modal::None;
    }

    pub fn open_data_dir(&self) -> io::Result<()> {
        open::that(&self.data_dir)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ui::root::update(ctx, frame, self);
    }
}

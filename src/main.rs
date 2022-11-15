// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod instances;

use color_eyre::Result;
use iced::{
    executor,
    widget::{button, column, row},
    Application, Command, Element, Settings, Theme,
};

pub fn main() -> Result<()> {
    color_eyre::install()?;
    IceLauncher::run(Settings::default())?;

    Ok(())
}

struct IceLauncher {
    current_view: View,
    instances_view: instances::InstancesView,
    about_view: about::AboutView,
}

#[derive(Debug, Clone)]
enum View {
    Instances,
    About,
}

#[derive(Debug, Clone)]
enum Message {
    ViewChanged(View),
}

impl Application for IceLauncher {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                current_view: View::Instances,
                instances_view: instances::InstancesView::new(),
                about_view: about::AboutView::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("🧊 Ice Launcher")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ViewChanged(view) => {
                self.current_view = view;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let navbar: Element<_> = column![
            button("Instances").on_press(Message::ViewChanged(View::Instances)),
            button("About").on_press(Message::ViewChanged(View::About))
        ]
        .into();

        let current_view: Element<_> = match self.current_view {
            View::Instances => self.instances_view.view(),
            View::About => self.about_view.view(),
        };

        row![navbar, current_view].into()
    }
}

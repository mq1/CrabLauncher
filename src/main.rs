// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod instances;
mod lib;
mod style;

use color_eyre::Result;
use iced::{
    executor,
    widget::{button, column, row},
    Application, Command, Element, Length, Settings, Theme,
};

const REPOSITORY: &str = "https://github.com/mq1/ice-launcher";
const LICENSE: &str = "https://github.com/mq1/ice-launcher/blob/main/COPYING";

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
pub enum View {
    Instances,
    About,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewChanged(View),
    OpenRepository,
    OpenLicense,
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
            Message::OpenRepository => {
                open::that(REPOSITORY).unwrap();
            }
            Message::OpenLicense => {
                open::that(LICENSE).unwrap();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let navbar: Element<_> = column![
            button("Instances")
                .on_press(Message::ViewChanged(View::Instances))
                .width(Length::Fill),
            button("About")
                .on_press(Message::ViewChanged(View::About))
                .width(Length::Fill),
        ]
        .spacing(10)
        .padding(20)
        .width(Length::Units(150))
        .into();

        let current_view: Element<_> = match self.current_view {
            View::Instances => self.instances_view.view(),
            View::About => self.about_view.view(),
        };

        row![navbar, current_view].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

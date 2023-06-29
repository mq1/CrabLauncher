// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use copypasta::{ClipboardContext, ClipboardProvider};
use iced::{
    widget::{button, column, text, vertical_space},
    Alignment, Command, Element, Length,
};

use crate::{pages::Page, style};

#[derive(Debug, Clone)]
pub enum Message {
    Login,
}

pub struct AddingAccount {
    pub url: String,
    pub code: String,
}

impl AddingAccount {
    pub fn new() -> Self {
        Self {
            url: String::new(),
            code: String::new(),
        }
    }
}

impl Page for AddingAccount {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Login => {
                open::that(&self.url).unwrap();

                // copy code to clipboard
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(self.code.to_owned()).unwrap();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let message = text(format!(
            "Please open up {} in a browser and put in the code {} to proceed with login",
            self.url, self.code
        ))
        .size(20);

        let open_button = button("Open page and copy code")
            .style(style::circle_button())
            .on_press(Message::Login);

        column![
            vertical_space(Length::Fill),
            message,
            open_button,
            vertical_space(Length::Fill),
        ]
        .width(Length::Fill)
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }
}

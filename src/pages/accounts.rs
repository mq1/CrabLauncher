// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    futures::TryFutureExt,
    widget::{button, column, container, row, scrollable, text},
    Command, Element, Length,
};
use iced_aw::FloatingElement;

use crate::{
    components::icons,
    pages::Page,
    style,
    util::accounts::{Account, Accounts},
};

#[derive(Debug, Clone)]
pub enum Message {
    GenerateDetails,
    AddAccount(
        (
            oauth2::basic::BasicClient,
            oauth2::devicecode::StandardDeviceAuthorizationResponse,
        ),
    ),
    AddingAccount(Result<Account, String>),
    SelectAccount(Account),
}

impl Page for Accounts {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut ret = Command::none();

        match message {
            Message::GenerateDetails => {
                ret = Command::perform(
                    async {
                        let client = Accounts::get_client().unwrap();
                        let details = Accounts::get_details(&client).unwrap();

                        (client, details)
                    },
                    Message::AddAccount,
                )
            }
            Message::AddAccount((client, details)) => {
                let client = Accounts::get_client().unwrap();
                let details = Accounts::get_details(&client).unwrap();

                ret = Command::perform(
                    Accounts::get_account(client, details).map_err(|e| e.to_string()),
                    Message::AddingAccount,
                )
            }
            Message::AddingAccount(account) => match account {
                Ok(account) => {
                    self.add_account(account).unwrap();
                }
                Err(e) => {
                    eprintln!("Error adding account: {e}");
                }
            },
            Message::SelectAccount(account) => {
                self.active = Some(account);
            }
        }

        ret
    }

    fn view(&self) -> Element<Self::Message> {
        let mut content = column![];

        if let Some(active_account) = &self.active {
            let row = row![
                text("Active account:"),
                text(active_account.mc_username.to_owned())
            ]
            .spacing(5);

            content = content.push(row);
        }

        for account in &self.others {
            let row = row![text(account.mc_username.to_owned()), button("Select")].spacing(5);

            content = content.push(row);
        }

        let content = scrollable(content).width(Length::Fill).height(Length::Fill);

        let content = FloatingElement::new(content, || {
            container(
                button(icons::plus())
                    .style(style::circle_button())
                    .on_press(Message::GenerateDetails),
            )
            .padding([0, 20, 20, 0])
            .into()
        });

        column![text("Accounts").size(30), content]
            .spacing(10)
            .padding(10)
            .into()
    }
}

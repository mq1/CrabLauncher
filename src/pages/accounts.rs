// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use copypasta::{ClipboardContext, ClipboardProvider};
use iced::{
    widget::{button, column, container, row, scrollable, text, vertical_space},
    Alignment, Command, Element, Length,
};
use iced_aw::FloatingElement;

use crate::{
    components::icons,
    pages::Page,
    style,
    util::accounts::{Account, Accounts},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    AddAccount,
    AddingAccount(Result<Account, String>),
    SelectAccount(Account),
    Login,
}

pub struct AccountsPage {
    pub accounts: Accounts,
    pub url: Option<String>,
    pub code: Option<String>,
}

impl AccountsPage {
    pub fn new(accounts: Accounts) -> Self {
        Self {
            accounts,
            url: None,
            code: None,
        }
    }
}

impl Page for AccountsPage {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut ret = Command::none();

        match message {
            Message::AddAccount => {
                let client = Accounts::get_client().unwrap();
                let details = Accounts::get_details(&client).unwrap();

                self.url = Some(details.verification_uri().to_string());
                self.code = Some(details.user_code().secret().to_string());

                ret = Command::perform(
                    async move { Accounts::get_account(client, details).map_err(|e| e.to_string()) },
                    Message::AddingAccount,
                )
            }
            Message::AddingAccount(account) => {
                self.url = None;
                self.code = None;

                match account {
                    Ok(account) => {
                        self.accounts.add_account(account).unwrap();
                    }
                    Err(e) => {
                        eprintln!("Error adding account: {e}");
                    }
                }
            }
            Message::SelectAccount(account) => {
                self.accounts.active = Some(account);
            }
            Message::Login => {
                if let (Some(url), Some(code)) = (&self.url, &self.code) {
                    println!("Please open up {} in a browser and put in the code {} to proceed with login", url, code);

                    open::that(url).unwrap();

                    // copy code to clipboard
                    let mut ctx = ClipboardContext::new().unwrap();
                    ctx.set_contents(code.to_owned()).unwrap();
                }
            }
        }

        ret
    }

    fn view(&self) -> Element<Self::Message> {
        if let (Some(url), Some(code)) = (&self.url, &self.code) {
            let message = text(format!(
                "Please open up {} in a browser and put in the code {} to proceed with login",
                url, code
            ))
            .size(20);

            let open_button = button("Open page and copy code")
                .style(style::circle_button())
                .on_press(Message::Login);

            return column![
                vertical_space(Length::Fill),
                message,
                open_button,
                vertical_space(Length::Fill),
            ]
            .width(Length::Fill)
            .spacing(10)
            .align_items(Alignment::Center)
            .into();
        }

        let mut content = column![];

        if let Some(active_account) = &self.accounts.active {
            let row = row![
                text("Active account:"),
                text(active_account.mc_username.to_owned())
            ]
            .spacing(5);

            content = content.push(row);
        }

        for account in &self.accounts.others {
            let row = row![text(account.mc_username.to_owned()), button("Select")].spacing(5);

            content = content.push(row);
        }

        let content = scrollable(content).width(Length::Fill).height(Length::Fill);

        let content = FloatingElement::new(content, || {
            container(
                button(icons::plus())
                    .style(style::circle_button())
                    .on_press(Message::AddAccount),
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

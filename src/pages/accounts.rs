// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use copypasta::{ClipboardContext, ClipboardProvider};
use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text, vertical_space},
    Alignment, Command, Element, Length,
};
use iced_aw::FloatingElement;
use rfd::MessageDialog;

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
    RemoveAccount(Account),
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
            Message::RemoveAccount(account) => {
                let yes = MessageDialog::new()
                    .set_title("Remove account")
                    .set_description(&format!(
                        "Are you sure you want to remove {}?",
                        account.mc_username
                    ))
                    .set_buttons(rfd::MessageButtons::YesNo)
                    .show();

                if yes {
                    self.accounts.remove_account(&account.mc_id).unwrap();
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
                .style(style::circle_button(theme::Button::Primary))
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

        let mut content = column![]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        if let Some(active_account) = &self.accounts.active {
            let row = row![
                text(&active_account.mc_username),
                horizontal_space(Length::Fill),
                button(icons::delete())
                    .on_press(Message::RemoveAccount(active_account.clone()))
                    .style(style::circle_button(theme::Button::Destructive)),
            ]
            .align_items(Alignment::Center)
            .padding(10);

            let active = container(row).style(style::card());
            content = content
                .push(text("Active account"))
                .push(active)
                .push(vertical_space(10));
        }

        if !self.accounts.others.is_empty() {
            let mut others = column![];

            for account in &self.accounts.others {
                let row = row![
                    text(&account.mc_username),
                    horizontal_space(Length::Fill),
                    button(icons::account_check())
                        .on_press(Message::SelectAccount(account.clone()))
                        .style(style::circle_button(theme::Button::Positive)),
                    button(icons::delete())
                        .on_press(Message::RemoveAccount(account.clone()))
                        .style(style::circle_button(theme::Button::Destructive)),
                ];

                others = others.push(row);
            }

            let others = scrollable(others);
            let others = container(others).style(style::card());

            content = content.push(text("Other accounts")).push(others);
        }

        let content = FloatingElement::new(content, || {
            container(
                button(icons::plus())
                    .style(style::circle_button(theme::Button::Primary))
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

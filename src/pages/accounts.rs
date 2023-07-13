// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use copypasta::{ClipboardContext, ClipboardProvider};
use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input,
        vertical_space,
    },
    Alignment, Command, Element, Length,
};
use iced_aw::{Card, FloatingElement, Modal};
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
    #[cfg(feature = "offline-accounts")]
    AddOfflineAccount,
    #[cfg(feature = "offline-accounts")]
    ChangeOfflineAccountUsername(String),
    #[cfg(feature = "offline-accounts")]
    AddingOfflineAccount,
    #[cfg(feature = "offline-accounts")]
    CloseAddOfflineAccount,
}

pub struct AccountsPage {
    pub accounts: Accounts,
    url: Option<String>,
    code: Option<String>,
    #[cfg(feature = "offline-accounts")]
    adding_offline_account: bool,
    #[cfg(feature = "offline-accounts")]
    offline_account_username: String,
}

impl AccountsPage {
    pub fn new(accounts: Accounts) -> Self {
        Self {
            accounts,
            url: None,
            code: None,
            #[cfg(feature = "offline-accounts")]
            adding_offline_account: false,
            #[cfg(feature = "offline-accounts")]
            offline_account_username: String::new(),
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
            #[cfg(feature = "offline-accounts")]
            Message::AddOfflineAccount => {
                self.adding_offline_account = true;
            }
            #[cfg(feature = "offline-accounts")]
            Message::ChangeOfflineAccountUsername(username) => {
                self.offline_account_username = username;
            }
            #[cfg(feature = "offline-accounts")]
            Message::AddingOfflineAccount => {
                self.adding_offline_account = false;
                let account = Account::new_offline(self.offline_account_username.clone());

                self.accounts.add_account(account).unwrap();
            }
            #[cfg(feature = "offline-accounts")]
            Message::CloseAddOfflineAccount => {
                self.adding_offline_account = false;
            }
            Message::SelectAccount(account) => {
                self.accounts.set_active_account(account).unwrap();
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
            let mut others = column![].spacing(10);

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
                ]
                .align_items(Alignment::Center)
                .padding(10)
                .spacing(5);

                others = others.push(container(row).style(style::card()));
            }

            content = content
                .push(text("Other accounts"))
                .push(scrollable(others));
        }

        #[cfg(feature = "offline-accounts")]
        let content = FloatingElement::new(content, || {
            row![
                button(
                    row![text(" Add offline account "), icons::plus()]
                        .align_items(Alignment::Center)
                )
                .on_press(Message::AddOfflineAccount)
                .style(style::circle_button(theme::Button::Secondary)),
                button(row![text(" Add account "), icons::plus()].align_items(Alignment::Center))
                    .on_press(Message::AddAccount)
                    .style(style::circle_button(theme::Button::Primary))
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .padding([0, 20, 20, 0])
            .into()
        });

        #[cfg(not(feature = "offline-accounts"))]
        let content = FloatingElement::new(content, || {
            container(
                button(row![text(" Add account "), icons::plus()].align_items(Alignment::Center))
                    .on_press(Message::AddAccount)
                    .style(style::circle_button(theme::Button::Primary)),
            )
            .padding([0, 20, 20, 0])
            .into()
        });

        let content = column![text("Accounts").size(30), content]
            .spacing(10)
            .padding(10);

        #[cfg(feature = "offline-accounts")]
        let content = Modal::new(self.adding_offline_account, content, || {
            Card::new(
                text("Add offline account"),
                column![
                    text("Enter your username"),
                    text_input("", &self.offline_account_username)
                        .width(Length::Fixed(300.))
                        .on_input(Message::ChangeOfflineAccountUsername)
                ]
                .spacing(10)
                .padding(10),
            )
            .foot(row![
                horizontal_space(Length::Fill),
                button("Add")
                    .on_press(Message::AddingOfflineAccount)
                    .style(style::circle_button(theme::Button::Primary))
                    .width(Length::Shrink)
                    .padding(10),
            ])
            .width(Length::Fixed(320.))
            .on_close(Message::CloseAddOfflineAccount)
            .into()
        });

        content.into()
    }
}

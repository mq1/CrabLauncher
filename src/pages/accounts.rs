// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    clipboard,
    Command,
    Element, Length, theme, widget::{button, Column, container, horizontal_space, Row, scrollable, text, text_input, vertical_space},
};
#[cfg(feature = "offline-accounts")]
use iced_aw::{Card, Modal};
use iced_aw::FloatingElement;
use rfd::MessageDialog;

use crate::{
    components::icons,
    pages::Page,
    style,
    types::generic_error::GenericError,
    util::accounts::{Account, Accounts},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    AddAccount,
    AddingAccount(Result<Account, GenericError>),
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
                    Accounts::get_account(client, details),
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
                    ret = clipboard::write(code.to_owned());
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

            return Column::new()
                .push(vertical_space(Length::Fill))
                .push(message)
                .push(open_button)
                .push(vertical_space(Length::Fill))
                .width(Length::Fill)
                .spacing(10)
                .align_items(Alignment::Center)
                .into();
        }

        let mut content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        if let Some(active_account) = &self.accounts.active {
            let row = Row::new()
                .push(text(&active_account.mc_username))
                .push(horizontal_space(Length::Fill))
                .push(
                    button(icons::view(icons::DELETE_OUTLINE))
                        .on_press(Message::RemoveAccount(active_account.clone()))
                        .style(style::circle_button(theme::Button::Destructive)),
                )
                .align_items(Alignment::Center)
                .padding(10);

            let active = container(row).style(style::card());
            content = content
                .push(text("Active account"))
                .push(active)
                .push(vertical_space(10));
        }

        if !self.accounts.others.is_empty() {
            let mut others = Column::new().spacing(10);

            for account in &self.accounts.others {
                let row = Row::new()
                    .push(text(&account.mc_username))
                    .push(horizontal_space(Length::Fill))
                    .push(button(icons::view(icons::ACCOUNT_CHECK_OUTLINE))
                        .on_press(Message::SelectAccount(account.clone()))
                        .style(style::circle_button(theme::Button::Positive)))
                    .push(
                        button(icons::view(icons::DELETE_OUTLINE))
                            .on_press(Message::RemoveAccount(account.clone()))
                            .style(style::circle_button(theme::Button::Destructive)),
                    )
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
            let add_offline_account_button = button(
                Row::new()
                    .push(text(" Add offline account "))
                    .push(icons::view(icons::ACCOUNT_PLUS_OUTLINE))
                    .align_items(Alignment::Center)
                    .padding(5)
            )
                .on_press(Message::AddOfflineAccount)
                .style(style::circle_button(theme::Button::Secondary));

            let add_account_button = button(
                Row::new()
                    .push(text(" Add account "))
                    .push(icons::view(icons::ACCOUNT_PLUS_OUTLINE))
                    .align_items(Alignment::Center)
                    .padding(5))
                .on_press(Message::AddAccount)
                .style(style::circle_button(theme::Button::Primary));

            Row::new()
                .push(add_offline_account_button)
                .push(add_account_button)
                .spacing(10)
                .align_items(Alignment::Center)
                .padding([0, 20, 20, 0])
                .into()
        });

        #[cfg(not(feature = "offline-accounts"))]
            let content = FloatingElement::new(content, || {
            let add_account_button = button(
                Row::new()
                    .push(text(" Add account "))
                    .push(icons::view(icons::ACCOUNT_PLUS_OUTLINE))
                    .align_items(Alignment::Center)
                    .padding(5))
                .on_press(Message::AddAccount)
                .style(style::circle_button(theme::Button::Primary));

            container(add_account_button)
                .padding([0, 20, 20, 0])
                .into()
        });

        let content = Column::new()
            .push(text("Accounts").size(30))
            .push(content)
            .spacing(10)
            .padding(10);

        #[cfg(feature = "offline-accounts")]
            let content = Modal::new(self.adding_offline_account, content, || {
            let foot = Row::new()
                .push(horizontal_space(Length::Fill))
                .push(button("Add")
                    .on_press(Message::AddingOfflineAccount)
                    .style(style::circle_button(theme::Button::Primary))
                    .width(Length::Shrink)
                    .padding(10));

            Card::new(
                text("Add offline account"),
                Column::new()
                    .push(text("Enter your username"))
                    .push(text_input("", &self.offline_account_username)
                        .width(Length::Fixed(300.))
                        .on_input(Message::ChangeOfflineAccountUsername))

                    .spacing(10)
                    .padding(10),
            )
                .foot(foot)
                .width(Length::Fixed(320.))
                .on_close(Message::CloseAddOfflineAccount)
                .into()
        });

        content.into()
    }
}

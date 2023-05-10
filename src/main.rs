// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod adding_account;
mod components;
mod instances;
mod latest_instance;
mod settings;
mod style;
mod util;

use std::{fs, path::PathBuf};

use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use directories::ProjectDirs;
use iced::{
    executor,
    futures::TryFutureExt,
    widget::{row, text},
    Application, Command, Element, Settings, Theme,
};
use once_cell::sync::Lazy;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "icy-launcher")
        .unwrap()
        .data_dir()
        .to_path_buf()
});

pub fn main() -> Result<()> {
    if !BASE_DIR.exists() {
        fs::create_dir_all(BASE_DIR.as_path())?;
    }

    App::run(Settings::default())?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    LatestInstance,
    Instances,
    NewInstance,
    Settings,
    About,
    Accounts,
    AddingAccount(String, String),
    FullscreenMessage(String),
}

struct App {
    view: View,
    instances: util::instances::Instances,
    settings: util::settings::Settings,
    accounts: util::accounts::Accounts,
    account_head: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    CheckForUpdates(bool),
    SaveSettings,
    OpenURL(String),
    AddAccount,
    Login(String, String),
    AddingAccount(Result<util::accounts::Account, String>),
    SelectAccount(util::accounts::Account),
    GotAccountHead(Result<Vec<u8>, String>),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instances = util::instances::Instances::load().unwrap();
        let settings = util::settings::Settings::load().unwrap();
        let accounts = util::accounts::Accounts::load().unwrap();

        let (command, account_head) = match accounts.active {
            Some(ref account) => {
                let account = account.clone();

                (
                    Command::perform(
                        util::accounts::get_head(account).map_err(|e| e.to_string()),
                        Message::GotAccountHead,
                    ),
                    Some(Vec::<u8>::new()),
                )
            }
            None => (Command::none(), None),
        };

        (
            Self {
                view: View::LatestInstance,
                instances,
                settings,
                accounts,
                account_head,
            },
            command,
        )
    }

    fn title(&self) -> String {
        String::from("Icy Launcher")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangeView(view) => {
                self.view = view;
                Command::none()
            }
            Message::CheckForUpdates(value) => {
                self.settings.check_for_updates = value;
                Command::none()
            }
            Message::SaveSettings => {
                self.settings.save().unwrap();
                Command::none()
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
                Command::none()
            }
            Message::AddAccount => {
                let client = util::accounts::get_client().unwrap();
                let details = util::accounts::get_details(&client).unwrap();

                self.view = View::AddingAccount(
                    details.verification_uri().to_string(),
                    details.user_code().secret().to_string(),
                );

                Command::perform(
                    util::accounts::get_account(client, details).map_err(|e| e.to_string()),
                    Message::AddingAccount,
                )
            }
            Message::Login(url, code) => {
                open::that(url).unwrap();

                // copy code to clipboard
                let mut ctx = ClipboardContext::new().unwrap();
                ctx.set_contents(code).unwrap();

                Command::none()
            }
            Message::AddingAccount(account) => {
                match account {
                    Ok(account) => {
                        self.accounts.add_account(account).unwrap();
                        self.view = View::Accounts;
                    }
                    Err(e) => {
                        self.view = View::FullscreenMessage(e);
                    }
                }

                Command::none()
            }
            Message::SelectAccount(account) => {
                self.accounts.set_active_account(account).unwrap();
                Command::none()
            }
            Message::GotAccountHead(result) => {
                match result {
                    Ok(head) => {
                        self.account_head = Some(head);
                    }
                    Err(e) => {
                        println!("Error getting account head: {}", e)
                    }
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let view = match &self.view {
            View::LatestInstance => latest_instance::view(),
            View::Instances => instances::view(&self.instances),
            View::NewInstance => text("New Instance").into(),
            View::Settings => settings::view(&self.settings),
            View::About => about::view(),
            View::Accounts => accounts::view(&self.accounts),
            View::AddingAccount(url, code) => adding_account::view(url, code),
            View::FullscreenMessage(message) => components::fullscreen_message::view(message),
        };

        if let View::AddingAccount(_, _) = self.view {
            return view;
        }

        if let View::FullscreenMessage(_) = self.view {
            return view;
        }

        let navbar = components::navbar::view(&self.view, &self.account_head);

        row![navbar, view].into()
    }
}

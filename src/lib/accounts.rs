// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use druid::{im::Vector, Data};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::BASE_DIR;

static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct Account {
    pub microsoft_refresh_token: String,
    pub minecraft_access_token: String,
    pub minecraft_username: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountsDocument {
    pub active_account: Option<String>,
    pub accounts: HashMap<String, Account>,
}

impl Default for AccountsDocument {
    fn default() -> Self {
        Self {
            active_account: None,
            accounts: HashMap::new(),
        }
    }
}

pub fn write(accounts: &AccountsDocument) -> Result<()> {
    let content = toml::to_string(accounts)?;
    fs::write(ACCOUNTS_PATH.as_path(), content)?;

    Ok(())
}

pub fn list() -> Result<Vector<(String, Account, bool)>> {
    if !ACCOUNTS_PATH.exists() {
        write(&AccountsDocument::default())?;
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let document: AccountsDocument = toml::from_str(&content)?;

    let mut accounts = Vector::new();
    for (id, account) in document.accounts.iter() {
        match document.active_account {
            Some(ref active_account) => accounts.push_back((
                id.to_owned(),
                account.to_owned(),
                active_account.to_owned() == id.to_owned(),
            )),
            None => accounts.push_back((id.to_owned(), account.to_owned(), false)),
        }
    }

    Ok(accounts)
}

pub fn get_active() -> Result<Option<(String, Account)>> {
    if !ACCOUNTS_PATH.exists() {
        write(&AccountsDocument::default())?;
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    let id = document.active_account;

    match id {
        Some(id) => {
            let account = document
                .accounts
                .remove(&id)
                .ok_or(anyhow!("Account not found"))?;
            Ok(Some((id, account)))
        }
        None => Ok(None),
    }
}

pub fn set_active(id: &str) -> Result<()> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    document.active_account = Some(id.to_owned());
    let content = toml::to_string(&document)?;
    fs::write(ACCOUNTS_PATH.as_path(), content)?;

    Ok(())
}

pub fn is_active(id: &str) -> Result<bool> {
    if !ACCOUNTS_PATH.exists() {
        write(&AccountsDocument::default())?;
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let document: AccountsDocument = toml::from_str(&content)?;

    match document.active_account {
        Some(active_account) => Ok(active_account == id),
        None => Ok(false),
    }
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::{eyre, Result};
use druid::im::Vector;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use smol::fs;

use super::{
    msa::{Account, AccountEntry},
    BASE_DIR,
};

static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

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

async fn write(accounts: &AccountsDocument) -> Result<()> {
    let content = toml::to_string(accounts)?;
    fs::write(ACCOUNTS_PATH.as_path(), content).await?;

    Ok(())
}

async fn read() -> Result<AccountsDocument> {
    if !ACCOUNTS_PATH.exists() {
        let default = AccountsDocument::default();

        write(&default);
        return Ok(default);
    }

    let content = fs::read(ACCOUNTS_PATH.as_path()).await?;
    let accounts = toml::from_slice(&content)?;

    Ok(accounts)
}

pub async fn list() -> Result<Vector<(AccountEntry, bool)>> {
    let document = read().await?;

    let mut accounts = Vector::new();
    for (id, account) in document.accounts.iter() {
        match document.active_account {
            Some(ref active_account) => accounts.push_back((
                AccountEntry {
                    minecraft_id: id.to_owned(),
                    account: account.to_owned(),
                },
                active_account.to_owned() == id.to_owned(),
            )),
            None => accounts.push_back((
                AccountEntry {
                    minecraft_id: id.to_owned(),
                    account: account.to_owned(),
                },
                false,
            )),
        }
    }

    Ok(accounts)
}

pub async fn get_active() -> Result<Option<AccountEntry>> {
    if !ACCOUNTS_PATH.exists() {
        write(&AccountsDocument::default()).await?;
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    let id = document.active_account;

    match id {
        Some(id) => {
            let account = document
                .accounts
                .remove(&id)
                .ok_or(eyre!("Account not found"))?;
            Ok(Some(AccountEntry {
                minecraft_id: id,
                account,
            }))
        }
        None => Ok(None),
    }
}

pub async fn set_active(id: &str) -> Result<()> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    document.active_account = Some(id.to_owned());
    let content = toml::to_string(&document)?;
    fs::write(ACCOUNTS_PATH.as_path(), content).await?;

    Ok(())
}

pub async fn add() -> Result<AccountEntry> {
    let msa = super::msa::login()?;
    let mut document = read().await?;
    document.accounts.insert(msa.minecraft_id.clone(), msa.account.clone());
    write(&document);

    let entry = AccountEntry {
        minecraft_id: msa.minecraft_id.clone(),
        account: msa.account.clone(),
    };

    Ok(entry)
}

pub async fn remove(id: &str) -> Result<()> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    document.accounts.remove(id);
    let content = toml::to_string(&document)?;
    fs::write(ACCOUNTS_PATH.as_path(), content).await?;

    Ok(())
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use smol::fs;

use super::{msa, BASE_DIR};

static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountsDocument {
    pub accounts: Vec<msa::Account>,
}

impl AsRef<AccountsDocument> for AccountsDocument {
    fn as_ref(&self) -> &Self {
        self
    }
}

async fn write<A: AsRef<AccountsDocument>>(accounts: A) -> Result<()> {
    let content = toml::to_string_pretty(accounts.as_ref())?;
    fs::write(ACCOUNTS_PATH.as_path(), content).await?;

    Ok(())
}

pub async fn read() -> Result<AccountsDocument> {
    if !ACCOUNTS_PATH.exists() {
        let default = AccountsDocument::default();

        smol::spawn(write(default.clone())).detach();
        return Ok(default);
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let accounts = toml::from_str(&content)?;

    Ok(accounts)
}

pub async fn update_accounts(accounts: Vec<msa::Account>) -> Result<()> {
    let mut document = read().await?;
    document.accounts = accounts;
    smol::spawn(write(document)).detach();

    Ok(())
}

pub async fn get_active() -> Result<Option<msa::Account>> {
    let document = read().await?;

    for account in document.accounts {
        if account.is_active {
            return Ok(Some(account));
        }
    }

    Ok(None)
}

pub async fn add() -> Result<msa::Account> {
    let account = msa::login().await?;
    let mut document = read().await?;
    document.accounts.push(account.clone());
    smol::spawn(write(document)).detach();

    Ok(account)
}

pub async fn remove(account: msa::Account) -> Result<()> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    document.accounts.retain(|a| a.mc_id != account.mc_id);
    write(document).await?;

    Ok(())
}

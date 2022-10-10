// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::eyre::Result;
use druid::im::Vector;
use oauth2::PkceCodeVerifier;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::{msa, BASE_DIR};

static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AccountsDocument {
    pub accounts: Vector<msa::Account>,
}

impl AsRef<AccountsDocument> for AccountsDocument {
    fn as_ref(&self) -> &Self {
        self
    }
}

async fn write(accounts: &AccountsDocument) -> Result<()> {
    let content = toml::to_string_pretty(accounts)?;
    fs::write(ACCOUNTS_PATH.as_path(), content).await?;

    Ok(())
}

pub async fn read() -> Result<AccountsDocument> {
    if !ACCOUNTS_PATH.exists() {
        let default = AccountsDocument::default();
        write(&default).await?;

        return Ok(default);
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let accounts = toml::from_str(&content)?;

    Ok(accounts)
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

pub async fn set_active(account: msa::Account) -> Result<()> {
    let mut document = read().await?;

    for a in document.accounts.iter_mut() {
        a.is_active = a.mc_id == account.mc_id;
    }

    write(&document).await?;

    Ok(())
}

pub async fn add(pkce_verifier: PkceCodeVerifier) -> Result<()> {
    let mut document = read().await?;
    let account = msa::listen_login_callback(pkce_verifier).await?;
    document.accounts.push_back(account);
    write(&document).await?;

    Ok(())
}

pub async fn remove(account: msa::Account) -> Result<()> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path()).await?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    document.accounts.retain(|a| a.mc_id != account.mc_id);
    write(&document).await?;

    Ok(())
}

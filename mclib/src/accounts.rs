// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::msa::AccountId;

use super::{msa, BASE_DIR};

static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

#[derive(Serialize, Deserialize, Default)]
pub struct AccountsDocument {
    pub active_account: Option<AccountId>,
    pub accounts: Vec<msa::Account>,
}

impl AccountsDocument {
    pub fn has_account_selected(&self) -> bool {
        self.active_account.is_some()
    }
}

fn write(accounts: &AccountsDocument) -> Result<()> {
    let content = toml::to_string_pretty(accounts)?;
    fs::write(ACCOUNTS_PATH.as_path(), content)?;

    Ok(())
}

pub fn read() -> Result<AccountsDocument> {
    if !ACCOUNTS_PATH.exists() {
        let default = AccountsDocument::default();
        write(&default)?;

        return Ok(default);
    }

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let accounts = toml::from_str(&content)?;

    Ok(accounts)
}

pub fn get_active() -> Result<Option<msa::Account>> {
    let document = read()?;

    if let Some(active_account) = document.active_account {
        let account = document
            .accounts
            .into_iter()
            .find(|account| account.mc_id == active_account);

        if let Some(account) = account {
            return Ok(Some(account));
        }
    }

    Ok(None)
}

pub fn set_active(account_id: AccountId) -> Result<()> {
    let mut document = read()?;
    document.active_account = Some(account_id);
    write(&document)?;

    Ok(())
}

pub fn add() -> Result<()> {
    let (auth_url, csrf_token, pkce_verifier) = msa::get_auth_url();
    open::that(auth_url.to_string())?;

    let mut document = read()?;
    let account = msa::listen_login_callback(csrf_token, pkce_verifier)?;
    document.accounts.push(account.unwrap());
    write(&document)?;

    Ok(())
}

pub fn remove(account: msa::Account) -> Result<()> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let mut document: AccountsDocument = toml::from_str(&content)?;
    document.accounts.retain(|a| a.mc_id != account.mc_id);
    write(&document)?;

    Ok(())
}

pub fn refresh(account: msa::Account) -> Result<msa::Account> {
    let account = msa::refresh(account)?;

    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let mut document: AccountsDocument = toml::from_str(&content)?;

    for account in document.accounts.iter_mut() {
        if account.mc_id == account.mc_id {
            *account = account.clone();
            break;
        }
    }

    write(&document)?;

    Ok(account)
}

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
    pub fn load() -> Result<Self> {
        if ACCOUNTS_PATH.exists() {
            let content = fs::read_to_string(&*ACCOUNTS_PATH)?;
            let doc = toml::from_str(&content)?;
            Ok(doc)
        } else {
            let doc = Self::default();
            doc.save()?;
            Ok(doc)
        }
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(&*ACCOUNTS_PATH, content)?;
        Ok(())
    }

    pub fn remove_account(&mut self, id: AccountId) -> Result<()> {
        self.accounts.retain(|a| a.mc_id != id);
        self.save()
    }

    pub fn add_account(&mut self, account: msa::Account) -> Result<()> {
        self.accounts.push(account);
        self.save()
    }

    pub fn set_active_account(&mut self, id: AccountId) -> Result<()> {
        self.active_account = Some(id);
        self.save()
    }

    // Refresh and return the account
    pub fn get_account(&mut self, id: &AccountId) -> Result<msa::Account> {
        let account = self
            .accounts
            .iter()
            .find(|a| &a.mc_id == id)
            .cloned()
            .unwrap();

        let refreshed_account = msa::refresh(account)?;

        for account in self.accounts.iter_mut() {
            if account.mc_id == refreshed_account.mc_id {
                *account = refreshed_account.clone();
                break;
            }
        }

        self.save()?;

        Ok(refreshed_account)
    }
}

pub fn login() -> Result<msa::Account> {
    let (auth_url, csrf_token, pkce_verifier) = msa::get_auth_url();
    open::that(auth_url.to_string())?;
    let account = msa::listen_login_callback(csrf_token, pkce_verifier)?.unwrap();

    Ok(account)
}

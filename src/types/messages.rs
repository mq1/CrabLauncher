// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::sync::Arc;

use anyhow::Error;

use crate::pages::Page;
use crate::subscriptions::download;
use crate::util::accounts::Account;
use crate::util::instances::Instance;
use crate::util::modrinth::Projects;

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page),
    Error(Arc<Error>, bool),
    OpenURL(String),
    GotUpdate(Result<Option<(String, String)>, Arc<Error>>),
    GotAccountHead(Result<Account, Arc<Error>>),
    UpdateInstances,
    CreatedInstance(Result<(), Arc<Error>>),
    LaunchInstance(Instance),
    DeleteInstance(Instance),
    DownloadProgressed(download::Progress),

    // Vanilla installer
    GetVersions,
    GotVersions(Result<Vec<String>, Arc<Error>>),
    ChangeName(String),
    SetOptimizeJvm(bool),
    SetMemory(String),
    SelectVersion(usize),
    CreateInstance,

    // Accounts
    AddAccount,
    LoggedIn(Result<Account, Arc<Error>>),
    SelectAccount(Account),
    RemoveAccount(Account),
    OpenLoginUrl,
    #[cfg(feature = "offline-accounts")]
    AddOfflineAccount,
    #[cfg(feature = "offline-accounts")]
    OfflineAccountUsernameChanged(String),

    // Settings
    SetCheckForUpdates(bool),
    SaveSettings,

    // Modrinth
    GetModpacks,
    GotModpacks(Result<Projects, Arc<Error>>),
}

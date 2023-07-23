// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::pages::Page;
use crate::subscriptions::download;
use crate::types::generic_error::GenericError;
use crate::util::accounts::Account;
use crate::util::instances::Instance;
use crate::util::modrinth::Projects;

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page),
    Error(GenericError, bool),
    OpenURL(String),
    GotUpdate(Result<Option<(String, String)>, GenericError>),
    GotAccountHead(Result<Account, GenericError>),
    UpdateInstances,
    CreatedInstance(Result<(), GenericError>),
    LaunchInstance(Instance),
    DeleteInstance(Instance),
    DownloadProgressed(download::Progress),

    // Vanilla installer
    GetVersions,
    GotVersions(Result<Vec<String>, GenericError>),
    ChangeName(String),
    SetOptimizeJvm(bool),
    SetMemory(String),
    SelectVersion(usize),
    CreateInstance,

    // Accounts
    AddAccount,
    LoggedIn(Result<Account, GenericError>),
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
    GotModpacks(Result<Projects, GenericError>),
}

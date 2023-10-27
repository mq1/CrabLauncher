// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::pages::Page;
use crate::subscriptions::download;
use lib::accounts::Account;
use lib::instances::Instance;
use lib::modrinth::Projects;

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Page),
    Error(String, bool),
    OpenURL(String),
    GotUpdate(Result<Option<(String, String)>, String>),
    GotAccountHead(Result<Account, String>),
    CreatedInstance(Result<(), String>),
    LaunchInstance(String),
    OpenInstanceFolder(String),
    OpenInstanceConfig(String),
    DeleteInstance(String),
    DownloadProgressed(download::Progress),

    // Vanilla installer
    GetVersions,
    GotVersions(Result<Vec<String>, String>),
    ChangeName(String),
    SetOptimizeJvm(bool),
    SetMemory(String),
    SelectVersion(usize),
    CreateInstance,

    // Accounts
    AddAccount,
    LoggedIn(Result<Account, String>),
    SelectAccount(Account),
    RemoveAccount(Account),
    OpenLoginUrl,
    AddOfflineAccount,
    OfflineAccountUsernameChanged(String),

    // Settings
    SetCheckForUpdates(bool),
    SaveSettings,

    // Modrinth
    GetModpacks,
    GotModpacks(Result<Projects, String>),
}

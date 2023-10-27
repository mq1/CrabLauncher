// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod adding_offline_account;
mod download;
mod error;
mod instances;
mod login;
mod modrinth_modpacks;
mod new_instance;
mod no_instances;
pub mod root;
mod settings;
mod status;
mod vanilla_installer;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Status(String),
    Error(String),
    Instances,
    NewInstance,
    VanillaInstaller,
    Settings,
    About,
    Accounts,
    AddingAccount,
    AddingOfflineAccount,
    Download,
    ModrinthModpacks,
}

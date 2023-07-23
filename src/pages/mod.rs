// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::types::generic_error::GenericError;

pub mod root;
mod about;
mod status;
mod instance;
mod error;
mod no_instances;
mod instances;
mod new_instance;
mod vanilla_installer;
mod accounts;
mod login;
mod settings;
mod modrinth_modpacks;
mod download;

#[cfg(feature = "offline-accounts")]
mod adding_offline_account;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Status(String),
    Error(GenericError),
    Instances,
    Instance(usize),
    LatestInstance,
    NewInstance,
    VanillaInstaller,
    Settings,
    About,
    Accounts,
    AddingAccount,
    #[cfg(feature = "offline-accounts")]
    AddingOfflineAccount,
    Download,
    ModrinthModpacks,
}

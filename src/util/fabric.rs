// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{types::generic_error::GenericError, util::DownloadItem};

pub fn install_version(
    minecraft_version: &str,
    fabric_version: &str,
) -> Result<Vec<DownloadItem>, GenericError> {
    Err(GenericError::Generic("Not implemented".to_string()))
}

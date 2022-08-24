use std::{collections::HashMap, path::PathBuf, fs};

use anyhow::Result;
use directories::ProjectDirs;
use druid::{im::Vector, Data};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref BASE_DIR: PathBuf = {
        let project_dirs = ProjectDirs::from("eu", "mq1", "ice-launcher").unwrap();
        project_dirs.config_dir().to_path_buf()
    };
    static ref ACCOUNTS_PATH: PathBuf = BASE_DIR.join("accounts.toml");
}

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct Account {
    pub microsoft_refresh_token: String,
    pub minecraft_access_token: String,
    pub minecraft_username: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountsDocument {
    pub accounts: HashMap<String, Account>,
}

pub fn list() -> Result<Vector<(String, Account)>> {
    let content = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let document: AccountsDocument = toml::from_str(&content)?;

    Ok(document.accounts.into_iter().collect())
}

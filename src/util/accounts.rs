// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use base64ct::{Base64UrlUnpadded, Encoding};
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use ureq::{Agent, AgentBuilder};

use crate::BASE_DIR;

const MSA_AUTHORIZATION_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_ENDPOINT: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const XBOXLIVE_AUTH_ENDPOINT: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZATION_ENDPOINT: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MINECRAFT_AUTH_ENDPOINT: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const MINECRAFT_PROFILE_ENDPOINT: &str = "https://api.minecraftservices.com/minecraft/profile";
const CLIENT_ID: &str = "ae26ac80-2153-4801-94f6-8859ce8e058a";
const SCOPE: &str = "XboxLive.signin offline_access";

#[cfg(dev)]
const REDIRECT_URI: &str = "http://localhost:1420/login";

#[cfg(not(dev))]
const REDIRECT_URI: &str = "tauri://localhost/login";

static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Account {
    pub ms_refresh_token: String,
    pub mc_id: String,
    pub mc_access_token: String,
    pub mc_username: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mc_username)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Accounts {
    pub active: Option<Account>,
    pub others: Vec<Account>,
}

impl Accounts {
    pub fn load() -> Result<Self> {
        if ACCOUNTS_PATH.exists() {
            let content = fs::read_to_string(&*ACCOUNTS_PATH)?;
            let doc = toml::from_str(&content)?;

            Ok(doc)
        } else {
            let doc: Accounts = Self::default();
            doc.save()?;

            Ok(doc)
        }
    }

    fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(&*ACCOUNTS_PATH, content)?;

        Ok(())
    }

    pub fn remove_account(&mut self, id: &str) -> Result<()> {
        if let Some(account) = &self.active {
            if account.mc_id == id {
                self.active = None;
            }
        } else {
            self.others.retain(|a| a.mc_id != id);
        }

        self.save()?;

        Ok(())
    }

    pub fn add_account(&mut self, account: Account) -> Result<()> {
        if self.active.is_none() {
            self.active = Some(account);
        } else {
            self.others.push(account);
        }

        self.save()?;

        Ok(())
    }

    pub fn set_active_account(&mut self, account: Account) -> Result<()> {
        if let Some(account) = &self.active {
            self.others.push(account.clone());
        }

        // Remove the account from the others list
        self.others.retain(|a| a.mc_id != account.mc_id);

        self.active = Some(account);

        self.save()?;

        Ok(())
    }

    // Refresh and return the active account
    pub fn get_account(&mut self, user_agent: &str) -> Result<Account> {
        let active_account = self.active.clone().unwrap();
        let refreshed_account = refresh(user_agent, active_account)?;

        self.active = Some(refreshed_account.clone());

        self.save()?;

        Ok(refreshed_account)
    }
}

fn get_code_challenge() -> (String, String) {
    let code_verifier = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect::<String>();

    let code_challenge = {
        let hash = Sha256::digest(code_verifier.as_bytes());
        let base64_hash = Base64UrlUnpadded::encode_string(&hash);

        base64_hash
    };

    (code_verifier, code_challenge)
}

fn get_state() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub fn get_auth_url() -> (String, String, String) {
    let state = get_state();
    let (code_verifier, code_challenge) = get_code_challenge();

    let url = format!(
        "{MSA_AUTHORIZATION_ENDPOINT}?client_id={CLIENT_ID}&response_type=code&redirect_uri={REDIRECT_URI}&response_mode=query&scope={SCOPE}&state={state}&code_challenge={code_challenge}&code_challenge_method=S256",
    );

    (url, state, code_verifier)
}

fn get_minecraft_account_data(
    agent: Agent,
    access_token: String,
    refresh_token: String,
) -> Result<Account, ureq::Error> {
    // Authenticate with Xbox Live

    #[derive(Deserialize)]
    struct XUI {
        uhs: String,
    }

    #[derive(Deserialize)]
    struct DisplayClaims {
        xui: Vec<XUI>,
    }

    #[derive(Deserialize)]
    struct XBLResponse {
        #[serde(rename = "Token")]
        token: String,
        #[serde(rename = "DisplayClaims")]
        display_claims: DisplayClaims,
    }

    let params = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", access_token),
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT",
    });

    println!("Authenticating with Xbox Live...");
    let xbl_response = agent
        .post(XBOXLIVE_AUTH_ENDPOINT)
        .set("Accept", "application/json")
        .send_json(&params)?
        .into_json::<XBLResponse>()?;
    println!("Authenticated with Xbox Live!");

    // Authenticate with XSTS

    #[derive(Deserialize)]
    struct XSTSResponse {
        #[serde(rename = "Token")]
        token: String,
    }

    let params = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": vec![xbl_response.token]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT",
    });

    println!("Authenticating with XSTS...");
    let xsts_response = agent
        .post(XSTS_AUTHORIZATION_ENDPOINT)
        .set("Accept", "application/json")
        .send_json(&params)?
        .into_json::<XSTSResponse>()?;
    println!("Authenticated with XSTS!");

    // Authenticate with Minecraft

    #[derive(Deserialize)]
    struct MinecraftResponse {
        access_token: String,
    }

    let params = json!({
        "identityToken":
            format!(
                "XBL3.0 x={};{}",
                xbl_response.display_claims.xui[0].uhs, xsts_response.token
            )
    });

    println!("Authenticating with Minecraft...");
    let minecraft_response = agent
        .post(MINECRAFT_AUTH_ENDPOINT)
        .set("Accept", "application/json")
        .send_json(&params)?
        .into_json::<MinecraftResponse>()?;
    println!("Authenticated with Minecraft!");

    // Get Minecraft profile

    #[derive(Deserialize)]
    struct MinecraftProfile {
        id: String,
        name: String,
    }

    let minecraft_profile = agent
        .get(MINECRAFT_PROFILE_ENDPOINT)
        .set(
            "Authorization",
            &format!("Bearer {}", minecraft_response.access_token),
        )
        .call()?
        .into_json::<MinecraftProfile>()?;

    let account = Account {
        ms_refresh_token: refresh_token,
        mc_id: minecraft_profile.id,
        mc_access_token: minecraft_response.access_token,
        mc_username: minecraft_profile.name,
    };

    Ok(account)
}

#[derive(Deserialize)]
struct OAuth2Token {
    access_token: String,
    refresh_token: String,
}

pub fn login(
    user_agent: &str,
    code: String,
    pkce_verifier: String,
) -> Result<Account, ureq::Error> {
    let agent = AgentBuilder::new().user_agent(user_agent).build();

    println!("Exchanging code for access token...");

    let params = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
        ("code_verifier", &pkce_verifier),
    ];

    let resp = agent
        .post(MSA_TOKEN_ENDPOINT)
        .set("Accept", "application/json")
        .send_form(&params)?
        .into_json::<OAuth2Token>()?;

    println!("Exchanged code for access token!");

    let entry = get_minecraft_account_data(agent, resp.access_token, resp.refresh_token)?;

    Ok(entry)
}

fn refresh(user_agent: &str, account: Account) -> Result<Account, ureq::Error> {
    let agent = AgentBuilder::new().user_agent(user_agent).build();

    let params = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("refresh_token", &account.ms_refresh_token),
        ("grant_type", "refresh_token"),
    ];

    let resp = agent
        .post(MSA_TOKEN_ENDPOINT)
        .set("Accept", "application/json")
        .send_form(&params)?
        .into_json::<OAuth2Token>()?;

    let entry = get_minecraft_account_data(agent, resp.access_token, resp.refresh_token)?;

    Ok(entry)
}

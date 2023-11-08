// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;
use std::{fs, io, thread};

use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use eframe::egui::Key::O;
use md5::{Digest, Md5};
use oauth2::ureq::http_client;
use oauth2::{
    basic::BasicClient, devicecode::StandardDeviceAuthorizationResponse, url, AuthUrl, ClientId,
    DeviceAuthorizationUrl, ExtraTokenFields, RefreshToken, Scope, StandardTokenResponse,
    TokenResponse, TokenType, TokenUrl,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{Duration, OffsetDateTime};

use crate::{AGENT, BASE_DIR};

pub static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));

pub const MSA_DEVICE_AUTH_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
pub const MSA_AUTHORIZATION_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
pub const MSA_TOKEN_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const XBOXLIVE_AUTH_ENDPOINT: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZATION_ENDPOINT: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MINECRAFT_AUTH_ENDPOINT: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const MINECRAFT_PROFILE_ENDPOINT: &str = "https://api.minecraftservices.com/minecraft/profile";
pub const CLIENT_ID: &str = "543a897a-0694-435b-a147-11de17aacd1f";
pub const SCOPES: &[&str] = &["XboxLive.signin"];

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Account {
    pub ms_refresh_token: Option<String>,
    pub mc_id: String,
    pub mc_access_token: Option<String>,
    pub mc_username: String,
    pub token_time: Option<OffsetDateTime>,
    pub cached_head_base64: Option<String>,
    pub cached_head_time: Option<OffsetDateTime>,
}

impl Account {
    pub fn new_offline(username: String) -> Self {
        let mc_id = {
            let text = format!("OfflinePlayer:{}", username);
            let hash = Md5::digest(text.as_bytes());
            hex::encode(hash)
        };

        Self {
            ms_refresh_token: None,
            mc_id,
            mc_access_token: None,
            mc_username: username,
            token_time: None,
            cached_head_base64: None,
            cached_head_time: None,
        }
    }

    pub async fn get_head(&self) -> Result<Account> {
        let mut account = self.clone();

        let now = OffsetDateTime::now_utc();

        if let Some(time) = &account.cached_head_time {
            if now < *time + Duration::minutes(5) {
                return Ok(account);
            }
        }

        let resp = AGENT
            .get(&format!("https://crafatar.com/avatars/{}", account.mc_id))
            .call()?;

        let mut bytes =
            Vec::with_capacity(resp.header("Content-Length").unwrap().parse::<usize>()?);
        io::copy(&mut resp.into_reader(), &mut bytes).unwrap();

        account.cached_head_base64 = Some(general_purpose::STANDARD_NO_PAD.encode(&bytes));
        account.cached_head_time = Some(now);

        Ok(account)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Accounts {
    pub active_mc_id: String,
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn load() -> Result<Self> {
        if !ACCOUNTS_PATH.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&*ACCOUNTS_PATH)?;
        let accounts = toml::from_str(&content)?;

        Ok(accounts)
    }

    fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(&*ACCOUNTS_PATH, content)?;

        Ok(())
    }

    pub fn remove_account(&mut self, id: &str) -> Result<()> {
        self.accounts.retain(|a| a.mc_id != id);
        self.save()?;

        Ok(())
    }

    pub fn add_account(&mut self, account: Account) -> Result<()> {
        self.accounts.push(account);
        self.save()?;

        Ok(())
    }

    pub fn set_active_account(&mut self, mc_id: &str) -> Result<()> {
        self.active_mc_id = mc_id.to_string();
        self.save()?;

        Ok(())
    }

    pub fn get_client() -> Result<BasicClient, url::ParseError> {
        let client_id = ClientId::new(CLIENT_ID.to_owned());
        let auth_url = AuthUrl::new(MSA_AUTHORIZATION_ENDPOINT.to_owned())?;
        let token_url = TokenUrl::new(MSA_TOKEN_ENDPOINT.to_owned())?;
        let device_auth_url = DeviceAuthorizationUrl::new(MSA_DEVICE_AUTH_ENDPOINT.to_owned())?;

        let client = BasicClient::new(client_id, None, auth_url, Some(token_url))
            .set_device_authorization_url(device_auth_url);

        Ok(client)
    }

    pub fn get_details(client: &BasicClient) -> Result<StandardDeviceAuthorizationResponse> {
        let scopes = SCOPES
            .iter()
            .map(|s| Scope::new(s.to_string()))
            .collect::<Vec<_>>();

        let details = client
            .exchange_device_code()?
            .add_scopes(scopes)
            .request(http_client)?;

        Ok(details)
    }

    pub async fn get_account(
        client: BasicClient,
        details: StandardDeviceAuthorizationResponse,
    ) -> Result<Account> {
        let token = client.exchange_device_access_token(&details).request(
            http_client,
            thread::sleep,
            None,
        )?;

        let now = OffsetDateTime::now_utc();

        let account = get_minecraft_account_data(&token, now)?;

        Ok(account)
    }

    pub fn update_account(&mut self, account: Account) -> Result<()> {
        for a in &mut self.accounts {
            if a.mc_id == account.mc_id {
                *a = account;
                break;
            }
        }

        self.save()?;

        Ok(())
    }

    pub fn refresh_account(&mut self, account: Account) -> Result<Account> {
        let now = OffsetDateTime::now_utc();

        if let Some(token_time) = account.token_time {
            if now < token_time + Duration::minutes(30) {
                return Ok(account);
            }
        }

        if let Some(refresh_token) = account.ms_refresh_token {
            let refresh_token = RefreshToken::new(refresh_token);

            let token = Self::get_client()?
                .exchange_refresh_token(&refresh_token)
                .request(http_client)?;

            let account = get_minecraft_account_data(&token, now)?;

            self.update_account(account.clone())?;

            return Ok(account);
        }

        Ok(account)
    }
}

pub fn get_minecraft_account_data<A: ExtraTokenFields, B: TokenType>(
    token: &StandardTokenResponse<A, B>,
    now: OffsetDateTime,
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
            "RpsTicket": format!("d={}", token.access_token().secret()),
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT",
    });

    println!("Authenticating with Xbox Live...");
    let xbl_response = AGENT
        .post(XBOXLIVE_AUTH_ENDPOINT)
        .set("Accept", "application/json")
        .send_json(params)?
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
    let xsts_response = AGENT
        .post(XSTS_AUTHORIZATION_ENDPOINT)
        .set("Accept", "application/json")
        .send_json(params)?
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
    let minecraft_response = AGENT
        .post(MINECRAFT_AUTH_ENDPOINT)
        .set("Accept", "application/json")
        .send_json(params)?
        .into_json::<MinecraftResponse>()?;
    println!("Authenticated with Minecraft!");

    // Get Minecraft profile

    #[derive(Deserialize)]
    struct MinecraftProfile {
        id: String,
        name: String,
    }

    let minecraft_profile = AGENT
        .get(MINECRAFT_PROFILE_ENDPOINT)
        .set(
            "Authorization",
            &format!("Bearer {}", minecraft_response.access_token),
        )
        .call()?
        .into_json::<MinecraftProfile>()?;

    let account = Account {
        ms_refresh_token: Some(token.refresh_token().unwrap().secret().to_string()),
        mc_id: minecraft_profile.id,
        mc_access_token: Some(minecraft_response.access_token),
        mc_username: minecraft_profile.name,
        token_time: Some(now),
        cached_head_base64: None,
        cached_head_time: None,
    };

    Ok(account)
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::Result;
use druid::Data;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use url::Url;

use super::USER_AGENT;

const MSA_AUTHORIZATION_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_ENDPOINT: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const XBOXLIVE_AUTH_ENDPOINT: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZATION_ENDPOINT: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MINECRAFT_AUTH_ENDPOINT: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const MINECRAFT_PROFILE_ENDPOINT: &str = "https://api.minecraftservices.com/minecraft/profile";
const CLIENT_ID: &str = "2000ea79-d993-4591-b9c4-e678f82ae1db";
const SCOPE: &str = "XboxLive.signin offline_access";
const REDIRECT_URI: &str = "http://127.0.0.1:3003/login";

static CODE_VERIFIER: Lazy<String> = Lazy::new(|| {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
});

static CODE_CHALLENGE: Lazy<String> = Lazy::new(|| {
    let hash = Sha256::digest(CODE_VERIFIER.as_bytes());
    base64_url::encode(&hash)
});

static STATE: Lazy<String> = Lazy::new(|| {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
});

pub static AUTH_URL: Lazy<Url> = Lazy::new(|| {
    let params = [
        ("client_id", CLIENT_ID),
        ("response_type", "code"),
        ("redirect_uri", REDIRECT_URI),
        ("response_mode", "query"),
        ("scope", SCOPE),
        ("state", STATE.as_ref()),
        ("code_challenge", CODE_CHALLENGE.as_ref()),
        ("code_challenge_method", "S256"),
    ];

    let url = Url::parse_with_params(MSA_AUTHORIZATION_ENDPOINT, &params).unwrap();

    url
});

#[derive(Deserialize)]
struct OAuth2Token {
    access_token: String,
    refresh_token: String,
}

async fn get_minecraft_account_data(msa_token: OAuth2Token) -> Result<Account> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

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
            "RpsTicket": format!("d={}", msa_token.access_token),
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT",
    });

    let xbl_response = client
        .post(XBOXLIVE_AUTH_ENDPOINT)
        .header("Accept", "application/json")
        .json(&params)
        .send()
        .await?
        .json::<XBLResponse>()
        .await?;

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

    let xsts_response = client
        .post(XSTS_AUTHORIZATION_ENDPOINT)
        .header("Accept", "application/json")
        .json(&params)
        .send()
        .await?
        .json::<XSTSResponse>()
        .await?;

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

    let minecraft_response = client
        .post(MINECRAFT_AUTH_ENDPOINT)
        .header("Accept", "application/json")
        .json(&params)
        .send()
        .await?
        .json::<MinecraftResponse>()
        .await?;

    // Get Minecraft profile

    #[derive(Deserialize)]
    struct MinecraftProfile {
        id: String,
        name: String,
    }

    let minecraft_profile = client
        .get(MINECRAFT_PROFILE_ENDPOINT)
        .bearer_auth(&minecraft_response.access_token)
        .send()
        .await?
        .json::<MinecraftProfile>()
        .await?;

    let account = Account {
        ms_refresh_token: msa_token.refresh_token,
        mc_id: minecraft_profile.id,
        mc_access_token: minecraft_response.access_token,
        mc_username: minecraft_profile.name,
        is_active: false,
    };

    Ok(account)
}

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct Account {
    pub ms_refresh_token: String,
    pub mc_id: String,
    pub mc_access_token: String,
    pub mc_username: String,
    pub is_active: bool,
}

pub async fn login(code: String) -> Result<Account> {
    let params = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
        ("code_verifier", CODE_VERIFIER.as_ref()),
    ];

    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

    let resp = client
        .post(MSA_TOKEN_ENDPOINT)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?
        .json::<OAuth2Token>()
        .await?;

    let entry = get_minecraft_account_data(resp).await?;

    Ok(entry)
}

pub async fn refresh(account: Account) -> Result<Account> {
    let params = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("refresh_token", &account.ms_refresh_token),
        ("grant_type", "refresh_token"),
    ];

    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

    let resp = client
        .post(MSA_TOKEN_ENDPOINT)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?
        .json::<OAuth2Token>()
        .await?;

    let entry = get_minecraft_account_data(resp).await?;

    Ok(entry)
}

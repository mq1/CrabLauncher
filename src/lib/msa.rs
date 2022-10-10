// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::Result;
use druid::Data;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

use super::HTTP_CLIENT;

const MSA_AUTHORIZATION_ENDPOINT: &str =
    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_ENDPOINT: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
const XBOXLIVE_AUTH_ENDPOINT: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_AUTHORIZATION_ENDPOINT: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const MINECRAFT_AUTH_ENDPOINT: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const MINECRAFT_PROFILE_ENDPOINT: &str = "https://api.minecraftservices.com/minecraft/profile";
const CLIENT_ID: &str = "ae26ac80-2153-4801-94f6-8859ce8e058a";
const REDIRECT_URI: &str = "http://127.0.0.1:3003/login";

static OAUTH2_CLIENT: Lazy<BasicClient> = Lazy::new(|| {
    BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        None,
        AuthUrl::new(MSA_AUTHORIZATION_ENDPOINT.to_string()).unwrap(),
        Some(TokenUrl::new(MSA_TOKEN_ENDPOINT.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string()).unwrap())
});

pub fn get_auth_url() -> (Url, PkceCodeVerifier) {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _) = OAUTH2_CLIENT
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("XboxLive.signin".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    (auth_url, pkce_verifier)
}

async fn get_minecraft_account_data(
    access_token: String,
    refresh_token: String,
) -> Result<Account> {
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

    let xbl_response = HTTP_CLIENT
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

    let xsts_response = HTTP_CLIENT
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

    let minecraft_response = HTTP_CLIENT
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

    let minecraft_profile = HTTP_CLIENT
        .get(MINECRAFT_PROFILE_ENDPOINT)
        .bearer_auth(&minecraft_response.access_token)
        .send()
        .await?
        .json::<MinecraftProfile>()
        .await?;

    let account = Account {
        ms_refresh_token: refresh_token,
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

async fn login(code: AuthorizationCode, pkce_verifier: PkceCodeVerifier) -> Result<Account> {
    let token_result = OAUTH2_CLIENT
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    let access_token = token_result.access_token().secret().clone();
    let refresh_token = token_result.refresh_token().unwrap().secret().clone();

    let entry = get_minecraft_account_data(access_token, refresh_token).await?;

    Ok(entry)
}

pub async fn refresh(account: Account) -> Result<Account> {
    let refresh_token = RefreshToken::new(account.ms_refresh_token.clone());

    let token_result = OAUTH2_CLIENT
        .exchange_refresh_token(&refresh_token)
        .request_async(async_http_client)
        .await?;

    let access_token = token_result.access_token().secret().clone();
    let refresh_token = token_result.refresh_token().unwrap().secret().clone();

    let entry = get_minecraft_account_data(access_token, refresh_token).await?;

    Ok(entry)
}

pub async fn listen_login_callback(pkce_verifier: PkceCodeVerifier) -> Result<Account> {
    let listener = TcpListener::bind("127.0.0.1:3003").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await.unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "You can close this tab now.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await.unwrap();

            return login(code, pkce_verifier).await;
        }
    }
}

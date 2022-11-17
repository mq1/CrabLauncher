// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use arrayvec::ArrayString;
use base64ct::{Base64UrlUnpadded, Encoding};
use color_eyre::eyre::{bail, Result};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use sha2::{Digest, Sha256};
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
const SCOPE: &str = "XboxLive.signin offline_access";

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

pub fn get_auth_url() -> (Url, String, String) {
    let state = get_state();
    let (code_verifier, code_challenge) = get_code_challenge();

    let params = [
        ("client_id", CLIENT_ID),
        ("response_type", "code"),
        ("redirect_uri", REDIRECT_URI),
        ("response_mode", "query"),
        ("scope", SCOPE),
        ("state", &state),
        ("code_challenge", &code_challenge),
        ("code_challenge_method", "S256"),
    ];

    let url = Url::parse_with_params(MSA_AUTHORIZATION_ENDPOINT, &params).unwrap();

    (url, state, code_verifier)
}

fn get_minecraft_account_data(access_token: String, refresh_token: String) -> Result<Account> {
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
    let xbl_response = HTTP_CLIENT
        .post(XBOXLIVE_AUTH_ENDPOINT)
        .header("Accept", "application/json")
        .json(&params)?
        .send()?
        .json::<XBLResponse>()?;
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
    let xsts_response = HTTP_CLIENT
        .post(XSTS_AUTHORIZATION_ENDPOINT)
        .header("Accept", "application/json")
        .json(&params)?
        .send()?
        .json::<XSTSResponse>()?;
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
    let minecraft_response = HTTP_CLIENT
        .post(MINECRAFT_AUTH_ENDPOINT)
        .header("Accept", "application/json")
        .json(&params)?
        .send()?
        .json::<MinecraftResponse>()?;
    println!("Authenticated with Minecraft!");

    // Get Minecraft profile

    #[derive(Deserialize)]
    struct MinecraftProfile {
        id: String,
        name: String,
    }

    let minecraft_profile = HTTP_CLIENT
        .get(MINECRAFT_PROFILE_ENDPOINT)
        .header(
            "Authorization",
            &format!("Bearer {}", minecraft_response.access_token),
        )
        .send()?
        .json::<MinecraftProfile>()?;

    let mut mc_id = ArrayString::<32>::new();
    mc_id.push_str(&minecraft_profile.id);

    let account = Account {
        ms_refresh_token: refresh_token,
        mc_id,
        mc_access_token: minecraft_response.access_token,
        mc_username: minecraft_profile.name,
    };

    Ok(account)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub ms_refresh_token: String,
    pub mc_id: ArrayString<32>,
    pub mc_access_token: String,
    pub mc_username: String,
}

#[derive(Deserialize)]
struct OAuth2Token {
    access_token: String,
    refresh_token: String,
}

fn login(code: String, pkce_verifier: String) -> Result<Account> {
    println!("Exchanging code for access token...");

    let params = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
        ("code_verifier", &pkce_verifier),
    ];

    let resp = HTTP_CLIENT
        .post(MSA_TOKEN_ENDPOINT)
        .header("Accept", "application/json")
        .form(&params)?
        .send()?
        .json::<OAuth2Token>()?;

    println!("Exchanged code for access token!");

    let entry = get_minecraft_account_data(resp.access_token, resp.refresh_token)?;

    Ok(entry)
}

pub fn refresh(account: Account) -> Result<Account> {
    let params = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("refresh_token", &account.ms_refresh_token),
        ("grant_type", "refresh_token"),
    ];

    let resp = HTTP_CLIENT
        .post(MSA_TOKEN_ENDPOINT)
        .header("Accept", "application/json")
        .form(&params)?
        .send()?
        .json::<OAuth2Token>()?;

    let entry = get_minecraft_account_data(resp.access_token, resp.refresh_token)?;

    Ok(entry)
}

pub fn listen_login_callback(csrf_token: String, pkce_verifier: String) -> Result<Option<Account>> {
    let listener = TcpListener::bind("127.0.0.1:3003")?;
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line)?;

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = value.into_owned();

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = value.into_owned();
            }

            if state != csrf_token {
                let message = "Invalid state";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes())?;

                bail!("Invalid CSRF token");
            }

            let message = "You can close this tab now";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;

            println!("Logging in...");
            let account = login(code, pkce_verifier)?;
            println!("Logged in!");
            return Ok(Some(account));
        }
    }

    Ok(None)
}

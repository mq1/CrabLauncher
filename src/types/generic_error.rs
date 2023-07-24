// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fmt, io, num, path::PathBuf};

use oauth2::http;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GenericError {
    Generic(String),
    NetworkError,
    IoError,
    ParseError,
    SerializeError,
    Oauth2Error,
    PathError(PathBuf),
    ExtractError,
    HashError(String, String),
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(msg) => write!(f, "{}", msg),
            Self::NetworkError => write!(f, "Network error"),
            Self::IoError => write!(f, "I/O error"),
            Self::ParseError => write!(f, "Parse error"),
            Self::SerializeError => write!(f, "Serialize error"),
            Self::Oauth2Error => write!(f, "Oauth2 error"),
            Self::PathError(path) => write!(f, "Path error: {}", path.display()),
            Self::ExtractError => write!(f, "Extract error"),
            Self::HashError(expected, got) => {
                write!(f, "Hash error: expected {}, got {}", expected, got)
            }
        }
    }
}

impl From<ureq::Error> for GenericError {
    fn from(error: ureq::Error) -> Self {
        dbg!(error);

        Self::NetworkError
    }
}

impl From<std::io::Error> for GenericError {
    fn from(error: io::Error) -> Self {
        dbg!(error);

        Self::IoError
    }
}

impl From<serde_json::Error> for GenericError {
    fn from(error: serde_json::Error) -> Self {
        dbg!(error);

        Self::ParseError
    }
}

impl From<toml::de::Error> for GenericError {
    fn from(error: toml::de::Error) -> Self {
        dbg!(error);

        Self::ParseError
    }
}

impl From<num::ParseIntError> for GenericError {
    fn from(error: num::ParseIntError) -> Self {
        dbg!(error);

        Self::ParseError
    }
}

impl From<toml::ser::Error> for GenericError {
    fn from(error: toml::ser::Error) -> Self {
        dbg!(error);

        Self::SerializeError
    }
}

impl From<oauth2::ConfigurationError> for GenericError {
    fn from(error: oauth2::ConfigurationError) -> Self {
        dbg!(error);

        Self::Oauth2Error
    }
}

impl<RE: std::error::Error, T: oauth2::ErrorResponse> From<oauth2::RequestTokenError<RE, T>>
for GenericError
{
    fn from(error: oauth2::RequestTokenError<RE, T>) -> Self {
        dbg!(error);

        Self::Oauth2Error
    }
}

impl From<oauth2::url::ParseError> for GenericError {
    fn from(error: oauth2::url::ParseError) -> Self {
        dbg!(error);

        Self::Oauth2Error
    }
}

impl From<zip::result::ZipError> for GenericError {
    fn from(error: zip::result::ZipError) -> Self {
        dbg!(error);

        Self::ExtractError
    }
}

impl From<http::status::InvalidStatusCode> for GenericError {
    fn from(error: http::status::InvalidStatusCode) -> Self {
        dbg!(error);

        Self::NetworkError
    }
}

impl From<http::header::InvalidHeaderValue> for GenericError {
    fn from(error: http::header::InvalidHeaderValue) -> Self {
        dbg!(error);

        Self::NetworkError
    }
}

impl From<http::Error> for GenericError {
    fn from(error: http::Error) -> Self {
        dbg!(error);

        Self::NetworkError
    }
}

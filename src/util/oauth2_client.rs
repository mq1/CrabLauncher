// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::io;

use oauth2::{HttpRequest, HttpResponse};
use oauth2::http::{HeaderMap, HeaderValue, Method, StatusCode};
use oauth2::http::header::{CONTENT_TYPE, InvalidHeaderValue};
use oauth2::http::status::InvalidStatusCode;
use thiserror::Error;

use crate::util::AGENT;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid header value {value:?} for header {name}")]
    InvalidHeaderValue {
        name: String,
        value: Vec<u8>,
    },
    #[error("Invalid status code {0}")]
    InvalidStatusCode(#[from] InvalidStatusCode),
    #[error("Invalid header value {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),
    #[error("HTTP error {0}")]
    HttpError(#[from] ureq::Error),
    #[error("UTF-8 error {0}")]
    Utf8Error(#[from] io::Error),
}

pub fn http_client(request: HttpRequest) -> Result<HttpResponse, Error> {
    let mut req = AGENT.request(
        request.method.as_str(),
        request.url.as_str(),
    );

    for (name, value) in request.headers {
        if let Some(name) = name {
            req = req.set(
                &name.to_string(),
                value.to_str().map_err(|_| Error::InvalidHeaderValue {
                    name: name.to_string(),
                    value: value.as_bytes().to_vec(),
                })?,
            );
        }
    }

    let resp = match request.method {
        Method::POST => req.send_bytes(&request.body),
        _ => req.call(),
    }?;

    let resp = HttpResponse {
        status_code: StatusCode::from_u16(resp.status())?,
        headers: vec![(
            CONTENT_TYPE,
            HeaderValue::from_str(resp.content_type())?,
        )]
            .into_iter()
            .collect::<HeaderMap>(),
        body: resp.into_string()?.as_bytes().into(),
    };

    Ok(resp)
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use oauth2::{HttpRequest, HttpResponse};
use oauth2::http::{HeaderMap, HeaderValue, Method, StatusCode};
use oauth2::http::header::CONTENT_TYPE;

use crate::types::generic_error::GenericError;
use crate::util::AGENT;

pub fn http_client(request: HttpRequest) -> Result<HttpResponse, GenericError> {
    let mut req = AGENT.request(
        request.method.as_str(),
        request.url.as_str(),
    );

    for (name, value) in request.headers {
        if let Some(name) = name {
            req = req.set(
                &name.to_string(),
                value.to_str().map_err(|_| {
                    GenericError::Generic(format!(
                        "invalid {} header value {:?}",
                        name,
                        value.as_bytes()
                    ))
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

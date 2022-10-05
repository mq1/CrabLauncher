// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use tokio::runtime::Runtime;
use warp::Filter;

use crate::{lib, AppState, View};

pub async fn serve(event_sink: druid::ExtEventSink) {
    let login = warp::path("login")
        .and(warp::query::<HashMap<String, String>>())
        .map(move |p: HashMap<String, String>| match p.get("code") {
            Some(code) => {
                let rt = Runtime::new().unwrap();
                let code = code.to_string();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    rt.block_on(lib::accounts::add(
                        code.to_owned(),
                        data.pkce_verifier.to_owned(),
                    ))
                    .unwrap();
                    let accounts = rt.block_on(lib::accounts::read()).unwrap().accounts;

                    data.accounts = accounts;
                    data.current_view = View::Accounts;
                });

                "You can close this tab"
            }
            None => "No \"code\" param in query",
        });

    warp::serve(login).run(([127, 0, 0, 1], 3003)).await;
}

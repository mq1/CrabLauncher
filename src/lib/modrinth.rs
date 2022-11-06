// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use druid::{im::Vector, Data};
use serde::Deserialize;

use crate::{AppState, View};

use super::HTTP_CLIENT;

#[derive(Deserialize, Data, Clone)]
pub struct Hit {
    pub title: String,
    pub versions: Vector<String>,
}

pub type Hits = Vector<Hit>;

#[derive(Deserialize)]
struct SearchResults {
    hits: Hits,
}

fn fetch_modpacks() -> Result<Hits> {
    let resp = HTTP_CLIENT
        .get("https://api.modrinth.com/v2/search?facets=[[\"project_type:modpack\"]]&limit=20")
        .send()?
        .json::<SearchResults>()?;

    Ok(resp.hits)
}

pub fn update_modpacks(event_sink: druid::ExtEventSink) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.modrinth_hits = Vector::new();
        data.current_message = "Fetching available Modrinth modpacks...".to_string();
        data.current_view = View::Loading;
    });

    let available_modpacks = fetch_modpacks()?;

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.modrinth_hits = available_modpacks;
        data.current_view = View::ModrinthModpackSelection;
    });

    Ok(())
}

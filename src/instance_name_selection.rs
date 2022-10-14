// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, TextBox},
    Color, LensExt, Widget, WidgetExt,
};
use futures_util::StreamExt;
use sha1::Sha1;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{
    lib::{
        self, check_hash, minecraft_assets::AssetIndex, minecraft_version_manifest::Version,
        minecraft_version_meta::MinecraftVersionMeta, HTTP_CLIENT,
    },
    AppState, NewInstanceState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("✍️ Type a name for your new instance").with_text_size(32.))
        .with_flex_spacer(1.)
        .with_child(
            TextBox::new()
                .with_placeholder("My new Instance")
                .lens(AppState::new_instance_state.then(NewInstanceState::instance_name))
                .padding(5.)
                .border(Color::GRAY, 1.)
                .rounded(5.)
                .expand_width(),
        )
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_child(Button::<AppState>::new("< Select version 📦").on_click(
                    |_, data, _| {
                        data.current_view = View::InstanceVersionSelection;
                    },
                ))
                .with_flex_spacer(1.)
                .with_child(Button::<AppState>::new("Done ✅").on_click(|ctx, data, _| {
                    let event_sink = ctx.get_external_handle();
                    let name = data.new_instance_state.instance_name.clone();
                    let version = data.new_instance_state.selected_version.clone().unwrap();

                    tokio::spawn(install_version(event_sink, name, version));
                    data.current_view = View::Progress;
                })),
        )
        .padding(10.)
}

async fn install_version(event_sink: druid::ExtEventSink, name: String, version: Version) {
    lib::instances::new(&name, &version).await.unwrap();
    let instance_list = lib::instances::list().await.unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = "Downloading version meta...".to_string();
        data.current_progress = 0.;
    });

    let meta_path = version.get_meta_path();
    let meta: MinecraftVersionMeta =
        if meta_path.exists() && check_hash::<Sha1>(&meta_path, &version.sha1) {
            let raw_meta = fs::read_to_string(meta_path).await.unwrap();

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = 1.;
            });

            serde_json::from_str(&raw_meta).unwrap()
        } else {
            let _ = fs::remove_file(&meta_path).await;

            let resp = HTTP_CLIENT.get(&version.url).send().await.unwrap();

            let size = resp.content_length().unwrap();
            let mut stream = resp.bytes_stream();

            fs::create_dir_all(meta_path.parent().unwrap()).await.unwrap();
            let mut file = File::create(&meta_path).await.unwrap();
            let mut meta = Vec::new();
            let mut downloaded_bytes = 0;

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.unwrap();
                file.write_all(&chunk).await.unwrap();
                meta.extend_from_slice(&chunk);
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / size as f64;
                });
            }

            serde_json::from_slice(&meta).unwrap()
        };

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = "Downloading assets...".to_string();
        data.current_progress = 0.;
    });

    let total_size = meta.asset_index.size + meta.asset_index.total_size.unwrap();
    let mut downloaded_bytes = 0;
    let index_path = meta.asset_index.get_path();

    let asset_index: AssetIndex =
        if index_path.exists() && check_hash::<Sha1>(&index_path, &meta.asset_index.sha1) {
            let raw_index = fs::read_to_string(index_path).await.unwrap();
            downloaded_bytes += meta.asset_index.size;

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_size as f64;
            });

            serde_json::from_str(&raw_index).unwrap()
        } else {
            let _ = fs::remove_file(&index_path).await;

            let resp = HTTP_CLIENT
                .get(meta.asset_index.url.clone())
                .send()
                .await
                .unwrap();

            let mut stream = resp.bytes_stream();
            let mut file = File::create(&index_path).await.unwrap();
            let mut raw_index = Vec::new();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.unwrap();
                file.write_all(&chunk).await.unwrap();
                raw_index.extend_from_slice(&chunk);
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / total_size as f64;
                });
            }

            serde_json::from_slice(&raw_index).unwrap()
        };

    // download all objects
    for object in asset_index.objects.values() {
        let path = object.get_path();
        if path.exists() && check_hash::<Sha1>(&path, &object.hash) {
            downloaded_bytes += object.size;

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_size as f64;
            });
        } else {
            let _ = fs::remove_file(&path).await;

            let resp = HTTP_CLIENT.get(object.get_url()).send().await.unwrap();

            let mut stream = resp.bytes_stream();
            let mut file = File::create(&path).await.unwrap();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.unwrap();
                file.write_all(&chunk).await.unwrap();
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / total_size as f64;
                });
            }
        }
    }

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = "Downloading libraries...".to_string();
        data.current_progress = 0.;
    });

    let mut downloaded_bytes = 0;
    let total_size = meta
        .libraries
        .iter()
        .map(|lib| lib.downloads.artifact.size)
        .sum::<usize>()
        + meta.downloads.client.size;

    for library in meta.libraries.iter() {
        let path = library.downloads.artifact.get_path();
        if path.exists() && check_hash::<Sha1>(&path, &library.downloads.artifact.sha1) {
            downloaded_bytes += library.downloads.artifact.size;

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_size as f64;
            });
        } else {
            let _ = fs::remove_file(&path).await;

            let resp = HTTP_CLIENT
                .get(&library.downloads.artifact.url)
                .send()
                .await
                .unwrap();

            let mut stream = resp.bytes_stream();
            let mut file = File::create(&path).await.unwrap();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk.unwrap();
                file.write_all(&chunk).await.unwrap();
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / total_size as f64;
                });
            }
        }
    }

    let path = meta.get_client_path();
    if path.exists() && check_hash::<Sha1>(&path, &meta.downloads.client.sha1) {
        downloaded_bytes += meta.downloads.client.size;

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_progress = downloaded_bytes as f64 / total_size as f64;
        });
    } else {
        let _ = fs::remove_file(&path).await;

        let resp = HTTP_CLIENT
            .get(&meta.downloads.client.url)
            .send()
            .await
            .unwrap();

        let mut stream = resp.bytes_stream();
        fs::create_dir_all(path.parent().unwrap()).await.unwrap();
        let mut file = File::create(&path).await.unwrap();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            file.write_all(&chunk).await.unwrap();
            downloaded_bytes += chunk.len();

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_size as f64;
            });
        }
    }

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.new_instance_state.available_minecraft_versions = Vector::new();
        data.instances = instance_list;
        data.current_view = View::Instances;
    });
}

// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{path::PathBuf, process::Stdio};

use druid::{im::Vector, Data};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use strum_macros::Display;
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

use color_eyre::eyre::Result;

use crate::{
    lib::{accounts, launcher_config, minecraft_assets::ASSETS_DIR},
    AppState, View,
};

use super::{
    check_hash,
    minecraft_assets::AssetIndex,
    minecraft_version_manifest::Version,
    minecraft_version_meta::{self, MinecraftVersionMeta},
    runtime_manager, BASE_DIR, HTTP_CLIENT,
};

// https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
const OPTIMIZED_FLAGS: &str = "-XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysActAsServerClassMachine -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:NmethodSweepActivity=1 -XX:ReservedCodeCacheSize=400M -XX:NonNMethodCodeHeapSize=12M -XX:ProfiledCodeHeapSize=194M -XX:NonProfiledCodeHeapSize=194M -XX:-DontCompileHugeMethods -XX:MaxNodeLimit=240000 -XX:NodeLimitFudgeFactor=8000 -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:ThreadPriorityPolicy=1 -XX:AllocatePrefetchStyle=3 -XX:+UseShenandoahGC -XX:ShenandoahGCMode=iu -XX:ShenandoahGuaranteedGCInterval=1000000 -XX:AllocatePrefetchStyle=1";

const INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("instances"));

#[derive(Display, Serialize, Deserialize, Clone, Data, PartialEq, Eq, Default)]
pub enum InstanceType {
    #[default]
    Vanilla,
    Fabric,
    Forge,
}

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct InstanceInfo {
    pub instance_type: InstanceType,
    pub minecraft_version: String,
    pub jre_version: u32,
}

impl Default for InstanceInfo {
    fn default() -> Self {
        Self {
            instance_type: InstanceType::default(),
            minecraft_version: "".to_string(),
            jre_version: 17,
        }
    }
}

#[derive(Clone, Data)]
pub struct Instance {
    pub name: String,
    pub info: InstanceInfo,
}

impl Instance {
    pub fn get_path(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }
}

async fn read_info(instance_name: &str) -> Result<InstanceInfo> {
    let path = INSTANCES_DIR.join(instance_name).join("instance.toml");
    let content = fs::read_to_string(path).await?;
    let info: InstanceInfo = toml::from_str(&content)?;

    Ok(info)
}

pub async fn list() -> Result<Vector<Instance>> {
    let mut instances = Vector::new();

    fs::create_dir_all(INSTANCES_DIR.as_path()).await?;
    let mut entries = fs::read_dir(INSTANCES_DIR.as_path()).await?;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.path().is_dir() {
            continue;
        }

        let instance_name = entry.file_name().into_string().unwrap();
        let info = read_info(&instance_name).await?;

        instances.push_back(Instance {
            name: instance_name,
            info,
        });
    }

    Ok(instances)
}

pub async fn new(
    instance_name: String,
    minecraft_version: Version,
    event_sink: druid::ExtEventSink,
) -> Result<()> {
    async fn create_instance(name: String, version_id: String) -> Result<Vector<Instance>> {
        let instance_dir = INSTANCES_DIR.join(name);
        fs::create_dir_all(&instance_dir).await?;

        let info = InstanceInfo {
            minecraft_version: version_id,
            ..Default::default()
        };

        let path = instance_dir.join("instance.toml");
        let content = toml::to_string_pretty(&info)?;
        fs::write(&path, content).await?;
        list().await
    }

    let instances = tokio::spawn(create_instance(
        instance_name.to_owned(),
        minecraft_version.id.to_owned(),
    ));

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.current_view = View::Progress;
        data.loading_message = "Downloading version meta...".to_string();
        data.current_progress = 0.;
    });

    let meta_path = minecraft_version.get_meta_path();
    let meta: MinecraftVersionMeta =
        if meta_path.exists() && check_hash::<Sha1>(&meta_path, &minecraft_version.sha1) {
            let raw_meta = fs::read_to_string(meta_path).await?;

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = 1.;
            });

            serde_json::from_str(&raw_meta)?
        } else {
            let _ = fs::remove_file(&meta_path).await;

            let mut resp = HTTP_CLIENT.get(&minecraft_version.url).send().await?;

            let size = resp.content_length().unwrap();

            fs::create_dir_all(meta_path.parent().unwrap()).await?;
            let mut file = File::create(&meta_path).await?;
            let mut meta = Vec::new();
            let mut downloaded_bytes = 0;

            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
                meta.extend_from_slice(&chunk);
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / size as f64;
                });
            }

            serde_json::from_slice(&meta)?
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
            let raw_index = fs::read_to_string(index_path).await?;
            downloaded_bytes += meta.asset_index.size;

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_size as f64;
            });

            serde_json::from_str(&raw_index)?
        } else {
            let _ = fs::remove_file(&index_path).await;

            let mut resp = HTTP_CLIENT.get(meta.asset_index.url.clone()).send().await?;

            fs::create_dir_all(index_path.parent().unwrap()).await?;

            let mut file = File::create(&index_path).await?;
            let mut raw_index = Vec::new();

            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
                raw_index.extend_from_slice(&chunk);
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / total_size as f64;
                });
            }

            serde_json::from_slice(&raw_index)?
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

            let mut resp = HTTP_CLIENT.get(object.get_url()?).send().await?;

            fs::create_dir_all(path.parent().unwrap()).await?;
            let mut file = File::create(&path).await?;

            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
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

            let mut resp = HTTP_CLIENT
                .get(&library.downloads.artifact.url)
                .send()
                .await?;

            fs::create_dir_all(path.parent().unwrap()).await?;
            let mut file = File::create(&path).await?;

            while let Some(chunk) = resp.chunk().await? {
                file.write_all(&chunk).await?;
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

        let mut resp = HTTP_CLIENT.get(&meta.downloads.client.url).send().await?;

        fs::create_dir_all(path.parent().unwrap()).await?;
        let mut file = File::create(&path).await?;

        while let Some(chunk) = resp.chunk().await? {
            file.write_all(&chunk).await?;
            downloaded_bytes += chunk.len();

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_size as f64;
            });
        }
    }

    let instances = instances.await??;

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.new_instance_state.available_minecraft_versions = Vector::new();
        data.instances = instances;
        data.current_view = View::Instances;
    });

    Ok(())
}

pub async fn remove(instance: Instance) -> Result<()> {
    let instance_dir = INSTANCES_DIR.join(instance.name);
    fs::remove_dir_all(&instance_dir).await?;

    Ok(())
}

pub async fn launch(instance: Instance, event_sink: druid::ExtEventSink) -> Result<()> {
    let instance_name = instance.name.clone();
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = format!("Running {}", instance_name);
        data.current_view = View::Loading;
    });

    let account = accounts::get_active().await?.unwrap();

    let config = launcher_config::read().await?;

    let version = minecraft_version_meta::get(&instance.info.minecraft_version).await?;

    /*
    let is_updated = runtime_manager::is_updated(&jre_version).await?;
    if !is_updated {
        println!("Installing JRE {}", jre_version);
        runtime_manager::install(&jre_version).await?;
    }
    */

    let java_path = runtime_manager::get_java_path(&instance.info.jre_version).await?;

    let mut jvm_args = vec![
        "-Dminecraft.launcher.brand=ice-launcher".to_string(),
        format!("-Dminecraft.launcher.version={}", env!("CARGO_PKG_VERSION")),
        format!("-Xmx{}", config.jvm_memory),
        format!("-Xms{}", config.jvm_memory),
        "-cp".to_string(),
        version.get_classpath(),
    ];

    #[cfg(target_os = "macos")]
    jvm_args.push("-XstartOnFirstThread".to_string());

    if config.automatically_optimize_jvm_arguments {
        jvm_args.extend(
            OPTIMIZED_FLAGS
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
    }

    let game_args = vec![
        "--username".to_string(),
        account.mc_username,
        "--version".to_string(),
        instance.info.minecraft_version.to_owned(),
        "--gameDir".to_string(),
        ".".to_string(),
        "--assetsDir".to_string(),
        ASSETS_DIR.to_string_lossy().to_string(),
        "--assetIndex".to_string(),
        version.assets,
        "--uuid".to_string(),
        account.mc_id,
        "--accessToken".to_string(),
        account.mc_access_token,
        "--clientId".to_string(),
        format!("ice-launcher/{}", env!("CARGO_PKG_VERSION")),
        "--userType".to_string(),
        "mojang".to_string(),
        "--versionType".to_string(),
        instance.info.instance_type.to_string(),
    ];

    let mut cmd = Command::new(java_path);

    cmd.stdout(Stdio::piped());

    let mut child = cmd
        .current_dir(instance.get_path())
        .args(jvm_args)
        .arg(version.main_class)
        .args(game_args)
        .spawn()
        .expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Instances;
        })
    });

    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }
    Ok(())
}

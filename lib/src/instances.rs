// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;
use std::{fs, process};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::accounts::Account;
use crate::paths::{ASSETS_DIR, INSTANCES_DIR};
use crate::{adoptium, vanilla_installer};

// https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
const OPTIMIZED_FLAGS: &str = " -XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysActAsServerClassMachine -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:NmethodSweepActivity=1 -XX:ReservedCodeCacheSize=400M -XX:NonNMethodCodeHeapSize=12M -XX:ProfiledCodeHeapSize=194M -XX:NonProfiledCodeHeapSize=194M -XX:-DontCompileHugeMethods -XX:MaxNodeLimit=240000 -XX:NodeLimitFudgeFactor=8000 -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:ThreadPriorityPolicy=1 -XX:AllocatePrefetchStyle=3 -XX:+UseShenandoahGC -XX:ShenandoahGCMode=iu -XX:ShenandoahGuaranteedGCInterval=1000000 -XX:AllocatePrefetchStyle=1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstanceInfo {
    last_played: String,
    pub minecraft: String,
    pub fabric: Option<String>,
    pub optimize_jvm: bool,
    pub memory: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub name: String,
    pub path: PathBuf,
    pub info: InstanceInfo,
}

impl Instance {
    pub fn launch(&self, account: &Account) -> Result<()> {
        let version_meta = vanilla_installer::VersionMeta::load(&self.info.minecraft)?;

        let java_path = adoptium::get_path("17")?;

        let mut jvm_flags = format!("-Xmx{0} -Xms{0}", self.info.memory);

        if self.info.optimize_jvm {
            jvm_flags.push_str(OPTIMIZED_FLAGS);

            if cfg!(target_os = "linux") {
                jvm_flags.push_str(" -XX:+UseTransparentHugePages");
            }
        }

        if cfg!(target_os = "macos") {
            jvm_flags.push_str(" -XstartOnFirstThread");
        }

        let mut child = process::Command::new(java_path)
            .current_dir(&self.path)
            .args(jvm_flags.split(' '))
            .arg("-cp")
            .arg(version_meta.get_classpath()?)
            .arg(format!(
                "-Dminecraft.launcher.brand={}",
                env!("CARGO_PKG_NAME")
            ))
            .arg(format!(
                "-Dminecraft.launcher.version={}",
                env!("CARGO_PKG_VERSION")
            ))
            .arg(version_meta.main_class)
            .arg("--username")
            .arg(&account.mc_username)
            .arg("--uuid")
            .arg(&account.mc_id)
            .arg("--accessToken")
            .arg(&account.mc_access_token)
            .arg("--userType")
            .arg("msa")
            .arg("--version")
            .arg(&self.info.minecraft)
            .arg("--gameDir")
            .arg(".")
            .arg("--assetsDir")
            .arg(ASSETS_DIR.to_string_lossy().to_string())
            .arg("--assetIndex")
            .arg(version_meta.assets)
            .arg("--versionType")
            .arg("release")
            .arg("--clientId")
            .arg(format!(
                "{}/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .spawn()?;

        println!("Launched instance: {}", self.name);

        child.wait()?;
        Ok(())
    }

    pub fn save_info(&self) -> Result<()> {
        let info_str = toml::to_string_pretty(&self.info)?;
        fs::write(self.path.join("instance.toml"), info_str)?;

        Ok(())
    }
    pub fn get_config_path(&self) -> PathBuf {
        self.path.join("instance.toml")
    }

    pub fn delete(&self) -> Result<()> {
        fs::remove_dir_all(&self.path).map_err(|e| e.into())
    }
}

pub fn list() -> Result<Vec<Instance>> {
    let mut list = Vec::new();

    for entry in fs::read_dir(&*INSTANCES_DIR)? {
        let entry = entry?;
        let path = entry.path();

        // Skip non-directories
        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let info = {
            let path = path.join("instance.toml");
            let info = fs::read_to_string(path)?;
            toml::from_str::<InstanceInfo>(&info)?
        };

        list.push(Instance { name, path, info });
    }

    // Sort by last played
    list.sort_by(|a, b| b.info.last_played.cmp(&a.info.last_played));

    Ok(list)
}

pub fn new(
    name: String,
    minecraft_version: String,
    fabric_version: Option<String>,
    optimize_jvm: bool,
    memory: String,
) -> Result<()> {
    let path = INSTANCES_DIR.join(&name);
    fs::create_dir(&path)?;

    let last_played = OffsetDateTime::now_utc().to_string();

    let info = InstanceInfo {
        last_played,
        minecraft: minecraft_version,
        fabric: fabric_version,
        optimize_jvm,
        memory,
    };
    let info_str = toml::to_string_pretty(&info)?;
    fs::write(path.join("instance.toml"), info_str)?;

    Ok(())
}

// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf, process};

use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{
    util::{accounts::Account, adoptium, vanilla_installer},
    ASSETS_DIR, BASE_DIR,
};

// https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
const OPTIMIZED_FLAGS: &str = "-XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysActAsServerClassMachine -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:NmethodSweepActivity=1 -XX:ReservedCodeCacheSize=400M -XX:NonNMethodCodeHeapSize=12M -XX:ProfiledCodeHeapSize=194M -XX:NonProfiledCodeHeapSize=194M -XX:-DontCompileHugeMethods -XX:MaxNodeLimit=240000 -XX:NodeLimitFudgeFactor=8000 -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:ThreadPriorityPolicy=1 -XX:AllocatePrefetchStyle=3 -XX:+UseShenandoahGC -XX:ShenandoahGCMode=iu -XX:ShenandoahGuaranteedGCInterval=1000000 -XX:AllocatePrefetchStyle=1";

// this works only if the launcher, java and javaw are started with admin privileges)
// not recommended, the flags are ignored if the launcher is not started with admin privileges
#[cfg(target_os = "windows")]
const OS_FLAGS: &str = "-XX:+UseLargePages -XX:LargePageSizeInBytes=2m";

#[cfg(target_os = "macos")]
const OS_FLAGS: &str = "-XstartOnFirstThread";

#[cfg(target_os = "linux")]
const OS_FLAGS: &str = "-XX:+UseTransparentHugePages";

pub static INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("instances");
    fs::create_dir_all(&dir).unwrap();

    dir
});

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceInfo {
    last_played: Option<DateTime<Utc>>,
    pub minecraft: String,
    fabric: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub name: String,
    pub info: InstanceInfo,
}

impl Instance {
    pub fn launch(&self, account: Account) -> Result<()> {
        let dir = INSTANCES_DIR.join(&self.name);
        let path = dir.join("instance.toml");
        let info = fs::read_to_string(path)?;
        let info = toml::from_str::<InstanceInfo>(&info)?;

        let version_meta = vanilla_installer::VersionMeta::load(&info.minecraft)?;

        let java_path = adoptium::get_path("17")?;

        let mut child = process::Command::new(java_path)
            .current_dir(dir)
            .args(OPTIMIZED_FLAGS.split(' '))
            .args(OS_FLAGS.split(' '))
            .arg("-Xmx4G") // TODO: Make this configurable
            .arg("-Xms4G") // TODO: Make this configurable
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
            .arg(account.mc_username)
            .arg("--uuid")
            .arg(account.mc_id)
            .arg("--accessToken")
            .arg(account.mc_access_token)
            .arg("--userType")
            .arg("msa")
            .arg("--version")
            .arg(info.minecraft)
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instances {
    pub list: Vec<Instance>,
}

impl Instances {
    fn sort(&mut self) {
        self.list.sort_by(|a, b| {
            let a = a
                .info
                .last_played
                .unwrap_or_else(|| DateTime::<Utc>::MIN_UTC);
            let b = b
                .info
                .last_played
                .unwrap_or_else(|| DateTime::<Utc>::MIN_UTC);

            b.cmp(&a)
        });
    }

    pub fn load() -> Result<Self> {
        if !INSTANCES_DIR.exists() {
            fs::create_dir(&*INSTANCES_DIR)?;

            return Ok(Self { list: Vec::new() });
        }

        let list = fs::read_dir(&*INSTANCES_DIR)?
            .filter_map(|entry| {
                let entry = entry.ok()?;

                // Skip non-directories
                if !entry.file_type().ok()?.is_dir() {
                    return None;
                }

                let path = entry.path();
                let name: String = path.file_name()?.to_str()?.to_string();

                let info_path = INSTANCES_DIR.join(&name).join("instance.toml");
                let info = fs::read_to_string(info_path).ok()?;
                let info = toml::from_str(&info).ok()?;

                let instance = Instance { name, info };

                Some(instance)
            })
            .collect();

        let mut instances = Self { list };
        instances.sort();

        Ok(instances)
    }

    pub fn new(
        &mut self,
        name: String,
        minecraft_version: String,
        fabric_version: Option<String>,
    ) -> Result<()> {
        let path = INSTANCES_DIR.join(&name);
        fs::create_dir(&path)?;

        let info = InstanceInfo {
            last_played: None,
            minecraft: minecraft_version,
            fabric: fabric_version,
        };
        let info_str = toml::to_string_pretty(&info)?;
        fs::write(path.join("instance.toml"), info_str)?;

        self.list.push(Instance { name, info });

        Ok(())
    }
}

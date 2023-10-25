// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, process};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::accounts::Account;
use crate::paths::{ASSETS_DIR, BASE_DIR};
use crate::{adoptium, vanilla_installer};

// https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
const OPTIMIZED_FLAGS: &str = " -XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysActAsServerClassMachine -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:NmethodSweepActivity=1 -XX:ReservedCodeCacheSize=400M -XX:NonNMethodCodeHeapSize=12M -XX:ProfiledCodeHeapSize=194M -XX:NonProfiledCodeHeapSize=194M -XX:-DontCompileHugeMethods -XX:MaxNodeLimit=240000 -XX:NodeLimitFudgeFactor=8000 -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:ThreadPriorityPolicy=1 -XX:AllocatePrefetchStyle=3 -XX:+UseShenandoahGC -XX:ShenandoahGCMode=iu -XX:ShenandoahGuaranteedGCInterval=1000000 -XX:AllocatePrefetchStyle=1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    last_played: String,
    pub minecraft: String,
    pub fabric: Option<String>,
    pub optimize_jvm: bool,
    pub memory: String,
}

#[derive(Debug, Clone)]
pub struct Instances {
    base_dir: PathBuf,
    pub list: HashMap<String, Instance>,
}

impl Instances {
    pub fn load() -> Result<Self> {
        let base_dir = BASE_DIR.join("instances");
        fs::create_dir_all(&base_dir)?;

        let mut list = HashMap::new();

        for entry in fs::read_dir(&base_dir)? {
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
                toml::from_str::<Instance>(&info)?
            };

            list.insert(name, info);
        }

        Ok(Self { base_dir, list })
    }

    pub fn get_dir(&self, name: &str) -> PathBuf {
        self.base_dir.join(name)
    }

    pub fn delete(&mut self, name: &str) -> Result<()> {
        let path = self.get_dir(name);
        fs::remove_dir_all(&path)?;

        self.list.remove(name);

        Ok(())
    }

    pub fn get_config_path(&self, name: &str) -> PathBuf {
        self.get_dir(name).join("instance.toml")
    }

    pub fn create(
        &mut self,
        name: String,
        minecraft_version: String,
        fabric_version: Option<String>,
        optimize_jvm: bool,
        memory: String,
    ) -> Result<()> {
        let path = self.get_dir(&name);
        fs::create_dir(&path)?;

        let last_played = OffsetDateTime::now_utc().to_string();

        let info = Instance {
            last_played,
            minecraft: minecraft_version,
            fabric: fabric_version,
            optimize_jvm,
            memory,
        };
        let info_str = toml::to_string_pretty(&info)?;
        fs::write(self.get_config_path(&name), info_str)?;

        self.list.insert(name, info);

        Ok(())
    }

    pub fn launch(&self, name: &str, account: &Account) -> Result<()> {
        let instance = self
            .list
            .get(name)
            .ok_or_else(|| anyhow!("Instance not found"))?;

        let version_meta = vanilla_installer::VersionMeta::load(&instance.minecraft)?;

        let java_path = adoptium::get_path("17")?;

        let mut jvm_flags = format!("-Xmx{0} -Xms{0}", instance.memory);

        if instance.optimize_jvm {
            jvm_flags.push_str(OPTIMIZED_FLAGS);

            if cfg!(target_os = "linux") {
                jvm_flags.push_str(" -XX:+UseTransparentHugePages");
            }
        }

        if cfg!(target_os = "macos") {
            jvm_flags.push_str(" -XstartOnFirstThread");
        }

        let mut child = process::Command::new(java_path)
            .current_dir(&self.get_dir(name))
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
            .arg(&instance.minecraft)
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

        println!("Launched instance: {}", name);

        child.wait()?;
        Ok(())
    }
}

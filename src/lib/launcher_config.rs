// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::eyre::Result;
use druid::{Data, Lens};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::BASE_DIR;

const LAUNCHER_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("config.toml"));

#[derive(Serialize, Deserialize, Data, Clone, Lens)]
pub struct LauncherConfig {
    pub automatically_check_for_updates: bool,
    pub jvm_arguments: String,
    pub jvm_memory: String,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            automatically_check_for_updates: true,
            // Arguments from https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
            jvm_arguments: "-server -XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:-DontCompileHugeMethods -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseStringDeduplication -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:+OmitStackTraceInFastThrow -XX:ThreadPriorityPolicy=1 -XX:MaxGCPauseMillis=40 -XX:+PerfDisableSharedMem -XX:G1HeapRegionSize=16M -XX:G1NewSizePercent=23 -XX:G1ReservePercent=20 -XX:SurvivorRatio=32 -XX:G1MixedGCCountTarget=3 -XX:G1HeapWastePercent=18 -XX:InitiatingHeapOccupancyPercent=10 -XX:G1RSetUpdatingPauseTimePercent=0 -XX:MaxTenuringThreshold=1 -XX:G1SATBBufferEnqueueingThresholdPercent=30 -XX:G1ConcMarkStepDurationMillis=5 -XX:G1ConcRSHotCardLimit=16 -XX:G1ConcRefinementServiceIntervalMillis=150 -XX:GCTimeRatio=99 -XX:AllocatePrefetchStyle=3".to_string(),
            jvm_memory: "2G".to_string(),
        }
    }
}

impl AsRef<LauncherConfig> for LauncherConfig {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub async fn write<L: AsRef<LauncherConfig>>(config: L) -> Result<()> {
    let content = toml::to_string_pretty(config.as_ref())?;
    fs::write(LAUNCHER_CONFIG_PATH.as_path(), content).await?;

    Ok(())
}

pub async fn read() -> Result<LauncherConfig> {
    if !LAUNCHER_CONFIG_PATH.exists() {
        write(&LauncherConfig::default()).await?;
    }

    let content = fs::read_to_string(LAUNCHER_CONFIG_PATH.as_path()).await?;
    let config: LauncherConfig = toml::from_str(&content)?;

    Ok(config)
}

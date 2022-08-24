use anyhow::anyhow;
use std::fs;

use anyhow::Result;
use directories::ProjectDirs;

pub fn list() -> Result<Vec<String>> {
    let project_dirs = ProjectDirs::from("eu", "mq1", "ice-launcher")
        .ok_or(anyhow!("Could not get project directories"))?;
    let base_dir = project_dirs.config_dir();
    let instances_dir = base_dir.join("instances");

    fs::create_dir_all(&instances_dir)?;

    let mut instances = Vec::new();
    for entry in fs::read_dir(instances_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let file_name = path.file_name().ok_or(anyhow!("Could not get file name"))?;
            let file_name = file_name
                .to_str()
                .ok_or(anyhow!("Could not convert file name to string"))?;
            instances.push(file_name.to_string());
        }
    }

    Ok(instances)
}

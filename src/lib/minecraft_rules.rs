// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
const CURRENT_OS: OsName = OsName::Linux;

#[cfg(target_os = "macos")]
const CURRENT_OS: OsName = OsName::MacOS;

#[cfg(target_os = "windows")]
const CURRENT_OS: OsName = OsName::Windows;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum Action {
    #[serde(rename = "allow")]
    Allow,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
enum OsName {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "osx")]
    MacOS,
    #[serde(rename = "windows")]
    Windows,
}

#[derive(Serialize, Deserialize)]
struct Os {
    name: OsName,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
enum Feature {
    #[serde(rename = "is_demo_user")]
    IsDemoUser,
    #[serde(rename = "has_custom_resolution")]
    HasCustomResolution,
}

#[derive(Serialize, Deserialize)]
pub struct Rule {
    action: Action,
    os: Option<Os>,
    features: Option<HashMap<Feature, bool>>,
}

pub fn is_rule_list_valid(rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if rule.features.is_some() {
            return false;
        }
        if let Some(os) = &rule.os {
            if rule.action == Action::Allow && os.name == CURRENT_OS {
                return true;
            }
        }
    }

    return false;
}

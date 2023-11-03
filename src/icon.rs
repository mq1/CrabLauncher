// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::svg;
use iced::{color, theme, Element};

use crate::message::Message;

#[derive(Debug, Clone)]
pub enum Icon {
    AccountPlusOutline,
    AccountAlertOutline,
    AccountCheckOutline,
    ArrowLeft,
    CogOutline,
    ContentSaveOutline,
    DeleteOutline,
    PackageVariant,
    ViewGridOutline,
    ViewGridPlusOutline,
    InformationOutline,
    RocketLaunchOutline,
    DownloadOutline,
    AlertCircleOutline,
    PlayOutline,
    FolderOpenOutline,
    Github,
    Minecraft,
    Modrinth,
}

impl Icon {
    pub fn view(&self, dimensions: u16) -> Element<Message> {
        let bytes: &[u8] = match self {
            Icon::AccountPlusOutline => {
                include_bytes!("../assets/mdi/account-plus-outline.svg")
            }
            Icon::AccountAlertOutline => {
                include_bytes!("../assets/mdi/account-alert-outline.svg")
            }
            Icon::AccountCheckOutline => {
                include_bytes!("../assets/mdi/account-check-outline.svg")
            }
            Icon::ArrowLeft => include_bytes!("../assets/mdi/arrow-left.svg"),
            Icon::CogOutline => include_bytes!("../assets/mdi/cog-outline.svg"),
            Icon::ContentSaveOutline => {
                include_bytes!("../assets/mdi/content-save-outline.svg")
            }
            Icon::DeleteOutline => include_bytes!("../assets/mdi/delete-outline.svg"),
            Icon::PackageVariant => include_bytes!("../assets/mdi/package-variant.svg"),
            Icon::ViewGridOutline => include_bytes!("../assets/mdi/view-grid-outline.svg"),
            Icon::ViewGridPlusOutline => {
                include_bytes!("../assets/mdi/view-grid-plus-outline.svg")
            }
            Icon::InformationOutline => {
                include_bytes!("../assets/mdi/information-outline.svg")
            }
            Icon::RocketLaunchOutline => {
                include_bytes!("../assets/mdi/rocket-launch-outline.svg")
            }
            Icon::DownloadOutline => include_bytes!("../assets/mdi/download-outline.svg"),
            Icon::AlertCircleOutline => {
                include_bytes!("../assets/mdi/alert-circle-outline.svg")
            }
            Icon::PlayOutline => include_bytes!("../assets/mdi/play-outline.svg"),
            Icon::FolderOpenOutline => {
                include_bytes!("../assets/mdi/folder-open-outline.svg")
            }
            Icon::Github => include_bytes!("../assets/simple-icons/github.svg"),
            Icon::Minecraft => include_bytes!("../assets/simple-icons/minecraft.svg"),
            Icon::Modrinth => include_bytes!("../assets/simple-icons/modrinth.svg"),
        };

        let handle = svg::Handle::from_memory(bytes);

        svg(handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xe2e8f0)),
            }))
            .width(dimensions)
            .height(dimensions)
            .into()
    }
}

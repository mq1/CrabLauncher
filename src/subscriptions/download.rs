// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::util::{download_file, DownloadItem};
use iced::{subscription, Subscription};

enum State {
    Ready(Vec<DownloadItem>),
    Downloading {
        items: Vec<DownloadItem>,
        total: usize,
        downloaded: usize,
    },
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

pub fn files(mut items: Vec<DownloadItem>) -> Subscription<Progress> {
    struct DownloadFiles;

    subscription::unfold(
        std::any::TypeId::of::<DownloadFiles>(),
        State::Ready(items),
        move |state| download(state),
    )
}

async fn download(state: State) -> (Progress, State) {
    match state {
        State::Ready(items) => (
            Progress::Advanced(0.0),
            State::Downloading {
                total: items.len(),
                items,
                downloaded: 0,
            },
        ),
        State::Downloading {
            mut items,
            total,
            downloaded,
        } => match items.pop() {
            Some(item) => {
                let downloaded = downloaded + 1;
                let percentage = (downloaded as f32 / total as f32) * 100.0;

                match download_file(&item) {
                    Ok(_) => (
                        Progress::Advanced(percentage),
                        State::Downloading {
                            items,
                            total,
                            downloaded,
                        },
                    ),
                    Err(_) => (Progress::Errored, State::Finished),
                }
            }
            None => (Progress::Finished, State::Finished),
        },
        State::Finished => iced::futures::future::pending().await,
    }
}

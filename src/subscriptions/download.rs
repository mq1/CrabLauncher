// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::util::DownloadQueue;
use iced::{subscription, Subscription};

enum State {
    Ready(DownloadQueue),
    Downloading {
        queue: DownloadQueue,
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

pub fn files(queue: DownloadQueue) -> Subscription<Progress> {
    struct DownloadFiles;

    subscription::unfold(
        std::any::TypeId::of::<DownloadFiles>(),
        State::Ready(queue),
        download,
    )
}

async fn download(state: State) -> (Progress, State) {
    match state {
        State::Ready(queue) => (
            Progress::Advanced(0.0),
            State::Downloading {
                total: queue.len(),
                queue,
                downloaded: 0,
            },
        ),
        State::Downloading {
            mut queue,
            total,
            downloaded,
        } => match queue.download_next() {
            Ok(true) => {
                let downloaded = downloaded + 1;
                let percentage = (downloaded as f32 / total as f32) * 100.0;

                (
                    Progress::Advanced(percentage),
                    State::Downloading {
                        queue,
                        total,
                        downloaded,
                    },
                )
            }
            Ok(false) => (Progress::Finished, State::Finished),
            Err(_) => (Progress::Errored, State::Finished),
        },
        State::Finished => iced::futures::future::pending().await,
    }
}

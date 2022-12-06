// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::collections::VecDeque;

use iced::{subscription, Subscription};
use mclib::DownloadItem;

enum State {
    Ready(VecDeque<DownloadItem>),
    Downloading {
        items: VecDeque<DownloadItem>,
        total: usize,
        downloaded: usize,
    },
    Finished,
}

#[derive(Debug, Clone)]
pub enum Event {
    Progress { percentage: f32, url: String },
    Errored,
    Finished,
}

pub fn files(items: Vec<DownloadItem>) -> Subscription<Event> {
    struct DownloadFiles;

    let items = VecDeque::from(items);

    subscription::unfold(
        std::any::TypeId::of::<DownloadFiles>(),
        State::Ready(items),
        |state| async move {
            match state {
                State::Ready(items) => {
                    let downloaded = 0;
                    let total = items.len();

                    (
                        Some(Event::Progress {
                            percentage: 0.0,
                            url: items[total - 1].url.to_string(),
                        }),
                        State::Downloading {
                            items,
                            downloaded,
                            total,
                        },
                    )
                }
                State::Downloading {
                    mut items,
                    total,
                    downloaded,
                } => {
                    let item = items.pop_front();

                    match item {
                        Some(item) => {
                            let downloaded = downloaded + 1;
                            let percentage = (downloaded as f32 / total as f32) * 100.0;

                            match item.download() {
                                Ok(_) => (
                                    Some(Event::Progress {
                                        percentage,
                                        url: item.url.to_string(),
                                    }),
                                    State::Downloading {
                                        items,
                                        downloaded,
                                        total,
                                    },
                                ),
                                Err(_) => (Some(Event::Errored), State::Finished),
                            }
                        }
                        None => (Some(Event::Finished), State::Finished),
                    }
                }
                State::Finished => iced::futures::future::pending().await,
            }
        },
    )
}

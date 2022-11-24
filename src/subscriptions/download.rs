// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs::File, io, path::PathBuf};

use iced::{subscription, Subscription};
use url::Url;

use crate::lib::HTTP_CLIENT;

#[derive(Debug, Clone)]
pub struct Item {
    url: Url,
    path: PathBuf,
}

enum State {
    Ready(Vec<Item>),
    Downloading {
        items: Vec<Item>,
        total: usize,
        downloaded: usize,
    },
    Finished,
}

#[derive(Debug, Clone)]
pub enum Event {
    Progress { percentage: f32, url: Url },
    Errored,
    Finished,
}

pub fn files(items: Vec<Item>) -> Subscription<Event> {
    struct DownloadFiles;

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
                            url: items[total - 1].url.clone(),
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
                    let item = items.pop();

                    match item {
                        Some(item) => {
                            let downloaded = downloaded + 1;
                            let percentage = (downloaded as f32 / total as f32) * 100.0;

                            match HTTP_CLIENT.get(&item.url).send() {
                                Ok(mut response) => {
                                    let mut file =
                                        File::create(item.path).expect("Could not create file");

                                    io::copy(&mut response, &mut file)
                                        .expect("Could not copy response to file");

                                    (
                                        Some(Event::Progress {
                                            percentage,
                                            url: item.url,
                                        }),
                                        State::Downloading {
                                            items,
                                            downloaded,
                                            total,
                                        },
                                    )
                                }
                                Err(_) => (
                                    Some(Event::Errored),
                                    State::Downloading {
                                        items,
                                        downloaded,
                                        total,
                                    },
                                ),
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

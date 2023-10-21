// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::Subscription;
use crate::subscriptions::download;
use crate::types::messages::Message;
use crate::util::DownloadQueue;

pub enum State {
    Idle,
    Downloading { progress: f32, queue: DownloadQueue },
    Finished,
    Errored,
}

pub struct Download {
    pub state: State,
}

impl Default for Download {
    fn default() -> Self {
        Self {
            state: State::Idle
        }
    }
}

impl Download {
    pub fn start(&mut self, queue: DownloadQueue) {
        match self.state {
            State::Idle { .. } | State::Finished { .. } | State::Errored { .. } => {
                self.state = State::Downloading {
                    progress: 0.0,
                    queue,
                };
            }
            _ => {}
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            State::Downloading { progress: _, queue } => {
                download::files(queue.clone()).map(Message::DownloadProgressed)
            }
            _ => Subscription::none(),
        }
    }

    pub fn update(&mut self, new_progress: download::Progress) {
        if let State::Downloading { progress, queue: _ } = &mut self.state {
            match new_progress {
                download::Progress::Started => {
                    *progress = 0.0;
                }
                download::Progress::Advanced(percentage) => {
                    *progress = percentage;
                }
                download::Progress::Finished => {
                    self.state = State::Finished;
                }
                download::Progress::Errored => {
                    self.state = State::Errored;
                }
            }
        }
    }
}

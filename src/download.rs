// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, progress_bar, text, Column},
    Alignment, Element, Subscription,
};
use url::Url;

use crate::{
    subscriptions::download::{self, Event, Item},
    Message,
};

#[derive(Debug)]
pub struct Download {
    state: State,
    items: Vec<Item>,
}

#[derive(Debug)]
enum State {
    Idle,
    Downloading {
        current_percentage: f32,
        current_url: Option<Url>,
    },
    Finished,
    Errored,
}

impl Download {
    pub fn new() -> Self {
        Download {
            state: State::Idle,
            items: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.state = State::Downloading {
            current_percentage: 0.0,
            current_url: None,
        };
    }

    pub fn update(&mut self, event: Event) {
        if let State::Downloading {
            current_percentage,
            current_url,
        } = &mut self.state
        {
            match event {
                Event::Progress { percentage, url } => {
                    *current_percentage = percentage;
                    *current_url = Some(url);
                }
                Event::Finished => {
                    self.state = State::Finished;
                }
                Event::Errored => {
                    self.state = State::Errored;
                }
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Downloading { .. } => {
                download::files(self.items.clone()).map(Message::DownloadEvent)
            }
            _ => Subscription::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let percentage = match &self.state {
            State::Idle { .. } => 0.0,
            State::Downloading {
                current_percentage,
                current_url: _,
            } => *current_percentage,
            State::Finished { .. } => 100.0,
            State::Errored { .. } => 0.0,
        };

        let info = match &self.state {
            State::Idle { .. } => "Idle".to_string(),
            State::Downloading {
                current_percentage: _,
                current_url,
            } => match current_url {
                Some(url) => format!("Downloading {url}"),
                None => "Downloading".to_string(),
            },
            State::Finished { .. } => "Finished".to_string(),
            State::Errored { .. } => "Errored".to_string(),
        };

        let progress_bar = progress_bar(0.0..=100.0, percentage);

        column![progress_bar, text(info),].into()
    }
}

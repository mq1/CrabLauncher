// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, container, horizontal_rule, scrollable, text, Column},
    Element,
};

use crate::{style, InstallerInfo, Message};

pub fn view(info: &InstallerInfo) -> Element<Message> {
    let content: Element<_> = match info.selected_modpack {
        Some(ref hit) => {
            let heading = text(&hit.title).size(50);

            let content: Element<_> = match &info.available_modpack_versions {
                Some(Ok(versions)) => {
                    let mut featured_list = Column::new().spacing(10);
                    let mut all_list = Column::new().spacing(10);

                    let featured_count = versions.iter().filter(|version| version.featured).count();

                    for (i, v) in versions.iter().enumerate() {
                        all_list = all_list.push(text(&v.name));

                        if i != versions.len() - 1 {
                            all_list = all_list.push(horizontal_rule(1));
                        }

                        if v.featured {
                            featured_list = featured_list.push(text(&v.name));

                            if i != featured_count - 1 {
                                featured_list = featured_list.push(horizontal_rule(1));
                            }
                        }
                    }

                    let featured_versions = container(
                        column![text("Featured").size(30), featured_list]
                            .spacing(10)
                            .padding(10),
                    )
                    .style(style::card());

                    let all_versions = container(
                        column![text("All").size(30), all_list]
                            .spacing(10)
                            .padding(10),
                    )
                    .style(style::card());

                    scrollable(column![featured_versions, all_versions].spacing(20)).into()
                }
                Some(Err(error)) => text(error).into(),
                None => text("Loading...").into(),
            };

            column![heading, content].spacing(20).into()
        }
        None => text("Loading...").into(),
    };

    container(content).padding(20).into()
}

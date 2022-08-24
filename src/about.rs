// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use const_format::formatcp;
use druid::{
    widget::{Button, Flex, Image, Label},
    ImageBuf, Widget, WidgetExt,
};

use crate::AppState;

const APP_VERSION: &str = formatcp!("Ice Launcher version {}", env!("CARGO_PKG_VERSION"));
const REPOSITORY: &str = "https://github.com/mq1/ice-launcher";
const LICENSE: &str = "https://github.com/mq1/ice-launcher/blob/main/COPYING";
const COPYRIGHT: &str = "Copyright © 2022 Manuel Quarneti";

pub fn build_widget() -> impl Widget<AppState> {
    let png_data = ImageBuf::from_data(include_bytes!("../ice-launcher.png")).unwrap();
    let image = Image::new(png_data).fix_width(128.);

    Flex::column()
        .with_flex_spacer(1.)
        .with_child(image)
        .with_default_spacer()
        .with_child(Label::new(APP_VERSION).with_text_size(32.))
        .with_default_spacer()
        .with_child(Label::new("Made with ❤️ by Manuel Quarneti").with_text_size(16.))
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_child(Button::new("Repository ↗️").on_click(
                    |_ctx, _data: &mut AppState, _env: &_| {
                        open::that(REPOSITORY).unwrap();
                    },
                ))
                .with_default_spacer()
                .with_child(Button::new("GPL-3.0-only Licensed ↗️").on_click(
                    |_ctx, _data: &mut AppState, _env: &_| {
                        open::that(LICENSE).unwrap();
                    },
                ))
                .with_flex_spacer(1.)
                .with_child(Label::new(COPYRIGHT)),
        )
        .padding(10.)
}

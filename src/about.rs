// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use const_format::formatcp;
use druid::{
    widget::{Button, Either, Flex, Image, Label},
    ImageBuf, Widget, WidgetExt,
};

use crate::AppState;

const APP_VERSION: &str = formatcp!("Ice Launcher version {}", env!("CARGO_PKG_VERSION"));
const REPOSITORY: &str = "https://github.com/mq1/ice-launcher";
const LATEST_RELEASE_URL: &str = "https://github.com/mq1/ice-launcher/releases/latest";
const LICENSE: &str = "https://github.com/mq1/ice-launcher/blob/main/COPYING";
const COPYRIGHT: &str = "Copyright © 2022 Manuel Quarneti";
const IMAGE_BYTES: &[u8] = include_bytes!("../ice-launcher.png");

pub fn build_widget() -> impl Widget<AppState> {
    let png_data = ImageBuf::from_data(IMAGE_BYTES).unwrap();
    let image = Image::new(png_data).fix_width(128.);

    let update_box = Flex::row()
        .with_child(Label::new("⚠️ Update available!"))
        .with_default_spacer()
        .with_child(Button::new("Update ↗️").on_click(update));

    let either = Either::new(
        |data: &AppState, _env: &_| data.is_update_available,
        update_box,
        Label::new("No updates available"),
    );

    Flex::column()
        .with_flex_spacer(1.)
        .with_child(image)
        .with_default_spacer()
        .with_child(Label::new(APP_VERSION).with_text_size(32.))
        .with_default_spacer()
        .with_child(Label::new("Made with ❤️ by Manuel Quarneti").with_text_size(16.))
        .with_default_spacer()
        .with_child(either)
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_child(Button::new("Repository ↗️").on_click(open_repository))
                .with_default_spacer()
                .with_child(Button::new("GPL-3.0-only Licensed ↗️").on_click(open_license))
                .with_flex_spacer(1.)
                .with_child(Label::new(COPYRIGHT)),
        )
        .padding(10.)
}

fn update(_: &mut druid::EventCtx, _: &mut AppState, _: &druid::Env) {
    open::that(LATEST_RELEASE_URL).unwrap();
}

fn open_repository(_: &mut druid::EventCtx, _: &mut AppState, _: &druid::Env) {
    open::that(REPOSITORY).unwrap();
}

fn open_license(_: &mut druid::EventCtx, _: &mut AppState, _: &druid::Env) {
    open::that(LICENSE).unwrap();
}

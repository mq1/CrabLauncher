// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{widget::text, Element};

use crate::Message;

pub fn view() -> Element<'static, Message> {
    text("Latest Instance").into()
}

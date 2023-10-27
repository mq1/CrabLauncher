// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::Row;
use iced::Element;

use crate::pages::Page;
use crate::types::launcher::Launcher;
use crate::types::messages::Message;
use crate::{components, pages};

pub fn view(launcher: &Launcher) -> Element<Message> {
    let navbar = components::navbar::view(launcher.name, &launcher.page, &launcher.accounts);

    let page_view = match &launcher.page {
        Page::Status(status) => pages::status::view(status),
        Page::Error(err) => pages::error::view(err),
        Page::About => pages::about::view(launcher.name),
        Page::Instances => pages::instances::view(&launcher.instances),
        Page::NewInstance => pages::new_instance::view(),
        Page::Accounts => pages::accounts::view(&launcher.accounts),
        Page::AddingAccount => pages::login::view(&launcher.login),
        Page::AddingOfflineAccount => {
            pages::adding_offline_account::view(&launcher.offline_account_username)
        }
        Page::VanillaInstaller => pages::vanilla_installer::view(&launcher.vanilla_installer),
        Page::Settings => pages::settings::view(&launcher.settings),
        Page::Download => pages::download::view(&launcher.download),
        Page::ModrinthModpacks => pages::modrinth_modpacks::view(&launcher.modrinth_modpacks),
    };

    Row::new().push(navbar).push(page_view).into()
}

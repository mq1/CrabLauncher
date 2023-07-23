// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::Element;
use iced::widget::Row;

use crate::{components, pages};
use crate::pages::Page;
use crate::types::download::Download;
use crate::types::login::Login;
use crate::types::messages::Message;
use crate::types::modrinth_modpacks::ModrinthModpacks;
use crate::types::vanilla_installer::VanillaInstaller;
use crate::util::accounts::Accounts;
use crate::util::instances::Instance;
use crate::util::settings::Settings;

pub fn view<'a>(
    page: &'a Page,
    launcher_name: &'static str,
    instances: &'a Vec<Instance>,
    login: &'a Login,
    accounts: &'a Accounts,
    offline_account_username: &'a str,
    vanilla_installer: &'a VanillaInstaller,
    settings: &'a Settings,
    download: &'a Download,
    modrinth_modpacks: &'a ModrinthModpacks,
) -> Element<'a, Message> {
    let navbar = components::navbar::view(launcher_name, page, accounts);

    let page_view = match page {
        Page::Status(status) => pages::status::view(status),
        Page::Error(err) => pages::error::view(err),
        Page::About => pages::about::view(launcher_name),
        Page::LatestInstance => match instances.first() {
            Some(instance) => pages::instance::view(instance),
            None => pages::no_instances::view(),
        },
        Page::Instance(i) => pages::instance::view(&instances[*i]),
        Page::Instances => pages::instances::view(instances),
        Page::NewInstance => pages::new_instance::view(),
        Page::Accounts => pages::accounts::view(accounts),
        Page::AddingAccount => pages::login::view(login),
        #[cfg(feature = "offline-accounts")]
        Page::AddingOfflineAccount => pages::adding_offline_account::view(offline_account_username),
        Page::VanillaInstaller => pages::vanilla_installer::view(vanilla_installer),
        Page::Settings => pages::settings::view(settings),
        Page::Download => pages::download::view(download),
        Page::ModrinthModpacks => pages::modrinth_modpacks::view(modrinth_modpacks),
    };

    Row::new()
        .push(navbar)
        .push(page_view)
        .into()
}

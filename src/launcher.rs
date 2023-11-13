// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::Row;
use iced::{executor, theme, Application, Color, Command, Element, Theme};

use crate::info::Info;
use crate::instances::Instances;
use crate::message::Message;
use crate::navbar::navbar;
use crate::pages::Page;
use crate::settings::Settings;
use crate::show_error;
use crate::vanilla_installer::VanillaInstaller;
use crate::version_manifest::VersionManifest;

pub struct Launcher {
    page: Page,
    instances: Instances,
    info: Info,
    vanilla_installer: VanillaInstaller,
    settings: Settings,
}

impl Application for Launcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let res = Instances::load();
        if let Err(error) = res {
            show_error(&error);
            std::process::exit(1);
        }
        let instances = res.unwrap();

        (
            Self {
                page: Page::Instances,
                instances,
                info: Info::new(),
                vanilla_installer: VanillaInstaller::new(),
                settings: Settings::load(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("CrabLauncher")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangePage(page) => {
                self.page = page;

                if self.page == Page::VanillaInstaller
                    && self.vanilla_installer.version_manifest.is_none()
                {
                    return Command::perform(
                        VersionManifest::fetch(),
                        Message::VersionManifestFetched,
                    );
                }
            }
            Message::ChangeVanillaInstallerName(name) => self.vanilla_installer.name = name,
            Message::VersionManifestFetched(result) => match result {
                Ok(version_manifest) => {
                    // find the index of the latest version
                    let latest_version = version_manifest
                        .versions
                        .iter()
                        .position(|version| version.id == version_manifest.latest.release)
                        .unwrap_or(0);

                    self.vanilla_installer.version_manifest = Some(version_manifest);
                    self.vanilla_installer.selected_version = Some(latest_version);
                }
                Err(error) => {
                    show_error(&error);
                }
            },
            Message::ChangeVanillaInstallerVersion(version) => {
                self.vanilla_installer.selected_version = Some(version);
            }
            Message::CreateVanillaInstance => {
                let version_manifest = self.vanilla_installer.version_manifest.as_ref().unwrap();
                let selected_version = self.vanilla_installer.selected_version.unwrap();

                let version = &version_manifest.versions[selected_version];

                let name = self.vanilla_installer.name.clone();

                if let Err(error) = self.instances.create(&name, &version.id) {
                    show_error(&error);
                }

                self.page = Page::Instances;
            }
            Message::SaveSettings => {
                if let Err(error) = self.settings.save() {
                    show_error(&error);
                }
            }
            Message::SetAutoUpdateCheck(auto_update_check) => {
                self.settings.auto_update_check = auto_update_check;
            }
            Message::ChangeJavaPath(java_path) => {
                self.settings.java_path = java_path;
            }
            Message::ChangeJavaMemory(java_memory) => {
                self.settings.java_memory = java_memory;
            }
            Message::OpenInstanceFolder(instance) => {
                if let Err(error) = self.instances.open_instance_dir(&instance) {
                    show_error(&error);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let navbar = navbar(&self.page);

        let content = match self.page {
            Page::Instances => self.instances.view(),
            Page::VanillaInstaller => self.vanilla_installer.view(),
            Page::Settings => self.settings.view(),
            Page::Info => self.info.view(),
        };

        Row::new().push(navbar).push(content).into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(theme::Palette {
            primary: Color::from_rgb8(192, 101, 33),
            ..Theme::Dark.palette()
        })
    }
}

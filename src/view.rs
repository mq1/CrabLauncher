// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{widget::ViewSwitcher, Widget};

use crate::{
    about, accounts, confirm_account_remove, confirm_instance_delete, instance_name_selection,
    instance_type_selection, instance_version_selection, instances, loading, modrinth_modpack,
    modrinth_modpack_selection, news, progress, settings, AppState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    ViewSwitcher::<AppState, View>::new(
        |data, _| data.current_view,
        |selector, _, _| match selector {
            View::Instances => Box::new(instances::build_widget()),
            View::InstanceTypeSelection => Box::new(instance_type_selection::build_widget()),
            View::Loading => Box::new(loading::build_widget()),
            View::Progress => Box::new(progress::build_widget()),
            View::InstanceVersionSelection => Box::new(instance_version_selection::build_widget()),
            View::InstanceNameSelection => Box::new(instance_name_selection::build_widget()),
            View::ConfirmInstanceDelete => Box::new(confirm_instance_delete::build_widget()),
            View::Accounts => Box::new(accounts::build_widget()),
            View::ConfirmAccountRemove => Box::new(confirm_account_remove::build_widget()),
            View::News => Box::new(news::build_widget()),
            View::Settings => Box::new(settings::build_widget()),
            View::About => Box::new(about::build_widget()),
            View::ModrinthModpackSelection => Box::new(modrinth_modpack_selection::build_widget()),
            View::ModrinthModpack => Box::new(modrinth_modpack::build_widget()),
        },
    )
}

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod about;
mod accounts;
mod instances;
mod lib;
mod news;
mod settings;

use std::thread;

use druid::{
    im::{vector, Vector},
    widget::{Button, Flex, ViewSwitcher},
    AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc,
};
use strum_macros::Display;

#[derive(PartialEq, Eq, Data, Clone, Copy, Display)]
enum View {
    Instances,
    Accounts,
    News,
    Settings,
    About,
}

#[derive(Data, Clone, Lens)]
pub struct AppState {
    current_view: View,
    instances: Vector<String>,
    news: Vector<(String, String)>,
}

fn main() {
    let window = WindowDesc::new(build_root_widget())
        .title("Ice Launcher")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        current_view: View::Instances,
        instances: Vector::from(lib::instances::list().unwrap()),
        news: vector![],
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("Launch failed");
}

fn build_root_widget() -> impl Widget<AppState> {
    let switcher_column = Flex::column()
        .with_child(
            Button::new("Instances").on_click(move |_ctx, data: &mut AppState, _env| {
                data.current_view = View::Instances;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("Accounts").on_click(move |_ctx, data: &mut AppState, _env| {
                data.current_view = View::Accounts;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("News").on_click(move |ctx, data: &mut AppState, _env| {
                if data.news.is_empty() {
                    let event_sink = ctx.get_external_handle();
                    thread::spawn(move || news::update_news(event_sink));
                }
                data.current_view = View::News;
            }),
        )
        .with_flex_spacer(1.)
        .with_child(
            Button::new("Settings").on_click(move |_ctx, data: &mut AppState, _env| {
                data.current_view = View::Settings;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("About").on_click(move |_ctx, data: &mut AppState, _env| {
                data.current_view = View::About;
            }),
        )
        .padding(10.)
        .background(Color::from_hex_str("#404040").unwrap());

    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            View::Instances => Box::new(instances::build_widget()),
            View::Accounts => Box::new(accounts::build_widget()),
            View::News => Box::new(news::build_widget()),
            View::Settings => Box::new(settings::build_widget()),
            View::About => Box::new(about::build_widget()),
        },
    );

    Flex::row()
        .with_child(switcher_column)
        .with_flex_child(view_switcher, 1.0)
        .expand_height()
}

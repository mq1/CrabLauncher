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
    widget::{Axis, Tabs},
    AppLauncher, Data, Lens, Widget, WindowDesc, WidgetExt,
};

#[derive(Data, Clone, Lens)]
pub struct AppState {
    instances: Vector<String>,
    news: Vector<(String, String)>,
}

fn main() {
    let window = WindowDesc::new(build_root_widget())
        .title("Ice Launcher")
        .window_size((800.0, 600.0));

    let launcher = AppLauncher::with_window(window);

    let event_sink = launcher.get_external_handle();

    thread::spawn(move || news::update_news(event_sink));

    let initial_state = AppState {
        instances: Vector::from(lib::instances::list().unwrap()),
        news: vector![]
    };

    launcher
        .log_to_console()
        .launch(initial_state)
        .expect("Launch failed");
}

fn build_root_widget() -> impl Widget<AppState> {
    Tabs::new()
        .with_axis(Axis::Vertical)
        .with_tab("Instances", instances::build_widget())
        .with_tab("Accounts", accounts::build_widget())
        .with_tab("News", news::build_widget())
        .with_tab("Settings", settings::build_widget())
        .with_tab("About", about::build_widget())
        .expand_width()
}

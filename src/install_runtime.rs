// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, RadioGroup, Scroll},
    Widget, WidgetExt,
};
use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    lib::{self, HTTP_CLIENT},
    AppState, View,
};

pub fn build_widget(available_runtimes: &Vector<i32>) -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("⬇️ Install runtime").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                RadioGroup::column(
                    available_runtimes
                        .iter()
                        .map(|r| (r.to_string(), Some(r.to_owned())))
                        .collect::<Vector<_>>(),
                )
                .expand_width()
                .lens(AppState::selected_runtime),
            )
            .vertical(),
        )
        .with_flex_spacer(1.)
        .with_child(Flex::row().with_flex_spacer(1.).with_child(
            Button::<AppState>::new("⬇️ Install").on_click(|ctx, data, _| {
                data.loading_message = "Downloading runtime...".to_string();
                data.current_progress = 0.;
                data.current_view = View::Progress;

                let runtime = data.selected_runtime.clone().unwrap();
                let event_sink = ctx.get_external_handle();
                tokio::spawn(install_runtime(event_sink, runtime));
            }),
        ))
        .padding(10.)
}

async fn install_runtime(event_sink: druid::ExtEventSink, runtime: i32) {
    let (package, download_path) = lib::runtime_manager::get_download(&runtime).await.unwrap();

    let mut stream = HTTP_CLIENT
        .get(package.link)
        .send()
        .await
        .unwrap()
        .bytes_stream();

    let mut file = File::create(&download_path).await.unwrap();
    let mut downloaded_bytes = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        file.write_all(&chunk).await.unwrap();
        downloaded_bytes += chunk.len();

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_progress = (downloaded_bytes / package.size) as f64;
        });
    }

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = "Installing runtime".to_string();
        data.current_view = View::Loading;
    });

    lib::runtime_manager::install(&download_path).await.unwrap();
    let list = lib::runtime_manager::list().await.unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.installed_runtimes = list;
        data.current_view = View::Runtimes;
    });
}

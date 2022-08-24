use druid::{
    widget::{CrossAxisAlignment, Flex, Label},
    Widget, WidgetExt,
};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("⚙️ Settings").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(Label::new("TODO").with_text_size(24.), 1.)
        .padding(10.)
}

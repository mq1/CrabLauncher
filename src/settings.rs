use druid::{widget::Label, Widget, WidgetExt};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Label::new("Settings").padding(10.)
}

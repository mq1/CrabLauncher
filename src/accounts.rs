use druid::{widget::Label, Widget, WidgetExt};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Label::new("Accounts").padding(10.)
}

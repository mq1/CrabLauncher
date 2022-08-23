use druid::{widget::Label, Widget};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Label::new("Accounts")
}

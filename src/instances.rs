use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    Widget, WidgetExt,
};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Instances").with_text_size(32.))
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(|name: &String, _env: &_| name.to_string()))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Launch"))
                })
                .with_spacing(10.)
                .lens(AppState::instances),
            )
            .vertical(),
            1.,
        )
        .padding(10.)
}

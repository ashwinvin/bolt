mod gui;
mod net;

use iced::widget::scrollable;
use iced::Application;
use iced::Settings;

pub struct BoltState {
    response: String,
    request: String,
    selected_method: Option<Method>,
    current_scroll_offset: scrollable::RelativeOffset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
}

fn main() {
    BoltState::run(Settings::default()).unwrap();
}

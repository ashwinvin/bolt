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

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::GET => "GET",
                Method::POST => "POST",
            }
        )
    }
}

impl Method {
    pub const ALL: [Method; 2] = [Method::GET, Method::POST];
}

fn main() {
    BoltState::run(Settings::default()).unwrap();
}

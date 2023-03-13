mod gui;
mod net;

use crate::gui::BoltState;
use iced::Application;
use iced::Settings;

fn main() {
    BoltState::run(Settings::default()).unwrap();
}

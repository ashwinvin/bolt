use crate::gui::interface::Message;

use crate::BoltState;
use iced::Command;

use crate::net::request::send_request;

pub fn process_update(sel: &mut BoltState, message: Message) -> Command<Message> {
    match message {
        Message::SendPressed => {
            if sel.request != "" {
                let resp = send_request(&sel.request, sel.selected_method.unwrap());

                sel.response = resp;
            }
        }

        Message::TextInputChanged(value) => {
            sel.request = value;
        }

        Message::ResponseInputChanged(_value) => {}

        Message::MethodSelected(meth) => {
            sel.selected_method = Some(meth);
        }

        Message::DocsPressed => {
            println!("Docs pressed");
        }

        Message::SettingsPressed => {
            println!("Settings pressed");
        }

        Message::Scrolled(offset) => {
            sel.current_scroll_offset = offset;
        }
    }

    return Command::none();
}

use crate::net::request::send_request;
use crate::BoltState;
use crate::Method;
use iced::widget::scrollable;
use iced::Command;

#[derive(Debug, Clone)]
pub enum Message {
    SendPressed,
    DocsPressed,
    SettingsPressed,
    TextInputChanged(String),
    MethodSelected(Method),
    Scrolled(scrollable::RelativeOffset),
    ResponseInputChanged(String),
}

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

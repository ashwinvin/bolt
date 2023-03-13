use iced::widget::button::Button;
use iced::widget::text_input::TextInput;
use iced::widget::{button, column, text};
use iced::{executor, Alignment, Application, Command, Element, Renderer, Settings, Theme};

struct BoltState {
    response: String,
    request: String,
}

#[derive(Debug, Clone)]
enum Message {
    SendPressed,
    TextInputChanged(String),
}

impl Application for BoltState {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let new_self = Self {
            response: String::from("Response body"),
            request: String::new(),
        };

        return (new_self, Command::none());
    }

    fn title(&self) -> String {
        return String::from("Bolt API Platform");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SendPressed => {
                let resp = get_body(&self.request);

                self.response = resp;
            }

            Message::TextInputChanged(value) => {
                self.request = value;
            }
        }

        return Command::none();
    }

    fn view(&self) -> Element<Message> {
        let text_box: TextInput<'_, Message, Renderer> =
            TextInput::new("http://", &self.request, Message::TextInputChanged);

        let submit: Button<'_, Message, Renderer> = button("Send").on_press(Message::SendPressed);

        let response = text(&self.response).size(20);

        let header = text("Bolt API").size(50);

        let final_view = column![header, text_box, submit, response,]
            .align_items(Alignment::Center)
            .into();

        return final_view;
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

fn get_body(url: &String) -> String {
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

    return resp;
}

fn main() {
    BoltState::run(Settings::default()).unwrap();
}

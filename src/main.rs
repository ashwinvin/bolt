use iced::widget::button::Button;
use iced::widget::text_input::TextInput;
use iced::widget::{button, column, container, pick_list, row, text};
use iced::{executor, Alignment, Application, Command, Element, Length, Renderer, Settings, Theme};

struct BoltState {
    response: String,
    request: String,
    selected_method: Option<Method>,
}

#[derive(Debug, Clone)]
enum Message {
    SendPressed,
    TextInputChanged(String),
    MethodSelected(Method),
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
    const ALL: [Method; 2] = [Method::GET, Method::POST];
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
            selected_method: Some(Method::GET),
        };

        return (new_self, Command::none());
    }

    fn title(&self) -> String {
        return String::from("Bolt API Platform");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SendPressed => {
                let resp = send_request(&self.request, self.selected_method.unwrap());

                self.response = resp;
            }

            Message::TextInputChanged(value) => {
                self.request = value;
            }

            Message::MethodSelected(meth) => {
                self.selected_method = Some(meth);
            }
        }

        return Command::none();
    }

    fn view(&self) -> Element<Message> {
        let method = pick_list(
            &Method::ALL[..],
            self.selected_method,
            Message::MethodSelected,
        )
        .placeholder("METHOD");

        let text_box: TextInput<'_, Message, Renderer> =
            TextInput::new("http://", &self.request, Message::TextInputChanged);

        let submit: Button<'_, Message, Renderer> = button("Send").on_press(Message::SendPressed);

        let response = text(&self.response).size(20);

        let name = row![text("Bolt API").size(30), text("Platform").size(30),];

        let extras = row![button("Docs"), button("Settings")];

        let header = row![name, extras]
            .width(Length::Fill)
            .height(70)
            .align_items(Alignment::Start);

        let request_panel = row![method, row![text_box].width(500), submit].height(200);

        let response_panel = column![response];

        let editor = column![request_panel, response_panel];

        let sidebar = column![button("API"), button("Collections"), button("Test"),].width(150);

        let body = row![sidebar, editor];

        let final_view: Element<Message> = column![header, body].into();

        return Element::from(
            container(final_view)
                .width(Length::Fill)
                .height(Length::Fill),
        );
    }

    fn theme(&self) -> Self::Theme {
        return Theme::Dark;
    }
}

fn send_request(url: &String, method: Method) -> String {
    match method {
        Method::GET => {
            let resp = reqwest::blocking::get(url).unwrap().text().unwrap();

            return resp;
        }

        Method::POST => {
            let client = reqwest::blocking::Client::new();
            let resp = client.post(url).send().unwrap().text().unwrap();

            return resp;
        }
    }
}

fn main() {
    BoltState::run(Settings::default()).unwrap();
}

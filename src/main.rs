use iced::widget::text_input::TextInput;
use iced::widget::{button, column, container, horizontal_space, pick_list, row, text};
use iced::Color;
use iced::{executor, theme, Alignment, Application, Command, Element, Length, Settings, Theme};

struct BoltState {
    response: String,
    request: String,
    selected_method: Option<Method>,
}

#[derive(Debug, Clone)]
enum Message {
    SendPressed,
    DocsPressed,
    SettingsPressed,
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

            Message::DocsPressed => {
                println!("Docs pressed");
            }

            Message::SettingsPressed => {
                println!("Settings pressed");
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

        let text_box = TextInput::new("http://", &self.request, Message::TextInputChanged);

        let submit = button("Send").on_press(Message::SendPressed);

        let response = text(&self.response)
            .size(20)
            .style(theme::Text::Color(Color::from_rgb(251.0, 87.0, 51.0)));

        let name = row![text("Bolt API").size(30),];

        let extras = row![
            button("Docs").on_press(Message::DocsPressed),
            horizontal_space(20),
            button("Settings").on_press(Message::SettingsPressed)
        ];

        let header = row![name, horizontal_space(Length::Fill), extras]
            .width(Length::Fill)
            .height(50)
            .align_items(Alignment::Start);

        let request_panel = row![
            horizontal_space(10),
            method,
            row![text_box].width(500),
            submit
        ]
        .height(200);

        let response_panel = column![response];

        let editor = column![request_panel, response_panel];

        let sidebar_first = column![
            button("API").width(100),
            button("Collections").width(100),
            button("Test").width(100),
        ]
        .height(Length::Fill)
        .width(100);

        let sidebar_sec = column![button("First API").width(200),]
            .height(Length::Fill)
            .width(200);

        let sidebar = row![sidebar_first, sidebar_sec]
            .height(Length::Fill)
            .width(300);

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

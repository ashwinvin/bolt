use iced::widget::scrollable::Properties;

use iced::widget::text_input::TextInput;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
    vertical_rule, vertical_space,
};
use iced::Color;
use iced::{
    executor, theme, Alignment, Application, Command, Element, Length, Renderer, Settings, Theme,
};

const JSON_LOREM: &str = r#"
  {
    "_id": "640eb7c4201230c379efbe84",
    "index": 0,
    "guid": "f949e9d0-291d-4d05-a116-f43b0569ed12",
    "isActive": true,
    "balance": "$3,550.89",
    "picture": "http://placehold.it/32x32",
    "age": 33,
    "eyeColor": "blue",
    "name": "Santana Randall",
    "gender": "male",
    "company": "COMVENE",
    "email": "santanarandall@comvene.com",
    "phone": "+1 (812) 478-2022",
    "address": "439 Farragut Road, Delco, Missouri, 8024",
    "about": "In ad enim magna ad excepteur occaecat commodo dolore aliqua ipsum sint laboris magna sunt. Irure duis sint dolor ea voluptate exercitation eu cillum. Amet minim aute consectetur in in velit nisi tempor non.\r\n",
    "registered": "2020-12-14T02:13:21 -01:00",
    "latitude": 87.197226,
    "longitude": 105.886306,
    "tags": [
      "minim",
      "fugiat",
      "quis",
      "minim",
      "nisi",
      "nisi",
      "consequat"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Gay Sweeney"
      },
      {
        "id": 1,
        "name": "Kelley Richards"
      },
      {
        "id": 2,
        "name": "Concetta Best"
      }
    ],
    "greeting": "Hello, Santana Randall! You have 6 unread messages.",
    "favoriteFruit": "strawberry"
  }
"#;

struct BoltState {
    response: String,
    request: String,
    selected_method: Option<Method>,
    current_scroll_offset: scrollable::RelativeOffset,
}

#[derive(Debug, Clone)]
enum Message {
    SendPressed,
    DocsPressed,
    SettingsPressed,
    TextInputChanged(String),
    MethodSelected(Method),
    Scrolled(scrollable::RelativeOffset),
    ResponseInputChanged(String),
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
            response: String::from(JSON_LOREM),
            request: String::new(),
            selected_method: Some(Method::GET),
            current_scroll_offset: scrollable::RelativeOffset::START,
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

            Message::ResponseInputChanged(_value) => {}

            Message::MethodSelected(meth) => {
                self.selected_method = Some(meth);
            }

            Message::DocsPressed => {
                println!("Docs pressed");
            }

            Message::SettingsPressed => {
                println!("Settings pressed");
            }

            Message::Scrolled(offset) => {
                self.current_scroll_offset = offset;
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

        let name = row![text("Bolt API").size(30),];

        let extras = row![
            button("Docs").on_press(Message::DocsPressed),
            horizontal_space(20),
            button("Settings").on_press(Message::SettingsPressed),
            horizontal_space(10),
        ];

        let header = row![name, horizontal_space(Length::Fill), extras]
            .width(Length::Fill)
            .height(35)
            .align_items(Alignment::Start);

        let request_bar = row![
            horizontal_space(10),
            method,
            row![text_box].width(500),
            submit
        ];

        let request_panel = column![vertical_space(10), request_bar].height(200);

        let response = text(&self.response)
            .size(20)
            .style(theme::Text::Color(Color::from_rgb(251.0, 87.0, 51.0)));

        // let response = TextInput::new(
        //     "Response body",
        //     &self.response,
        //     Message::ResponseInputChanged,
        // );

        let scroller: iced::widget::scrollable::Scrollable<'_, Message, Renderer> =
            scrollable(column![response].width(Length::Fill).spacing(40))
                .height(Length::Fill)
                .vertical_scroll(Properties::new().width(5).scroller_width(5))
                .on_scroll(Message::Scrolled);

        let response_panel = column![scroller];

        let editor = column![request_panel, horizontal_rule(5), response_panel];

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

        let sidebar = row![
            sidebar_first,
            vertical_rule(10),
            sidebar_sec,
            vertical_rule(10)
        ]
        .height(Length::Fill)
        .width(300);

        let body = row![sidebar, editor];

        let final_view: Element<Message> =
            column![vertical_space(5), header, horizontal_rule(5), body].into();

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

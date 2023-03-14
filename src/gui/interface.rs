use crate::gui::update::process_update;
use crate::gui::update::Message;
use crate::gui::view::get_view;
use crate::BoltState;
use crate::Method;

use iced::widget::scrollable;

use iced::{executor, Application, Command, Element, Theme};

impl Application for BoltState {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let new_self = Self {
            response: String::from(JSON_LOREM_IPSUM),
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
        return process_update(self, message);
    }

    fn view(&self) -> Element<Message> {
        return get_view(self);
    }

    fn theme(&self) -> Self::Theme {
        return Theme::Dark;
    }
}

const JSON_LOREM_IPSU: &str = r#"
  {
    "_id": "640eb7c4201230c379efbe84",
    "index": 0,
    "guid": "f949e9d0-291d-4d05-a116-f43b0569ed12",
    "isActive": true,
   "favoriteFruit": "strawberry"
  }
"#;

const JSON_LOREM_IPSUM: &str = r#"
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

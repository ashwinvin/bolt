use crate::gui::interface::Message;
use crate::BoltState;
use crate::Method;
use iced::widget::scrollable::Properties;

use iced::widget::text_input::TextInput;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
    vertical_rule, vertical_space,
};
use iced::Color;
use iced::{theme, Alignment, Element, Length, Renderer};

// use crate::net::request::send_request;

pub fn get_view(sel: &BoltState) -> Element<Message> {
    let method = pick_list(
        &Method::ALL[..],
        sel.selected_method,
        Message::MethodSelected,
    )
    .placeholder("METHOD");

    let text_box = TextInput::new("http://", &sel.request, Message::TextInputChanged);

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

    let response = text(&sel.response)
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

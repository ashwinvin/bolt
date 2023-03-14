use crate::gui::update::Message;
use crate::BoltState;
use crate::Method;
use iced::widget::scrollable::Properties;

use iced::widget::text_input::TextInput;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
    vertical_rule, vertical_space, Column, Container, Row,
};
use iced::Color;
use iced::{theme, Alignment, Element, Length, Renderer};

#[derive(Default)]
enum CustomContainerTheme {
    #[default]
    DefaultBG,
    CustomBG(Color),
}

impl iced::widget::container::StyleSheet for CustomContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        match self {
            Self::DefaultBG => iced::widget::container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
                ..Default::default()
            },

            Self::CustomBG(color) => iced::widget::container::Appearance {
                background: Some(iced::Background::Color(*color)),
                ..Default::default()
            },
        }
    }
}

fn get_header<'a>() -> Row<'a, Message> {
    let name = row![text("Bolt").size(30)];

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

    return header;
}

fn get_sidebar<'a>() -> Container<'a, Message> {
    let sidebar_first = column![
        button("API").width(100),
        button("Collections").width(100),
        button("Test").width(100),
    ]
    .width(100);

    let sidebar_sec = column![button("First API").width(200),].width(200);

    let sidebar = row![
        sidebar_first,
        column![vertical_rule(1)],
        sidebar_sec,
        column![vertical_rule(1)],
    ]
    .height(Length::Shrink)
    .width(300);

    let sidebar = Container::new(column![sidebar]).style(iced::theme::Container::Custom(Box::new(
        CustomContainerTheme::CustomBG(Color::from_rgb(38.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0)),
    )));
    return sidebar;
}

fn get_request_panel<'a>(sel: &BoltState) -> Container<'a, Message> {
    let method = pick_list(
        &Method::ALL[..],
        sel.selected_method,
        Message::MethodSelected,
    )
    .placeholder("METHOD");

    let text_box = TextInput::new("http://", &sel.request, Message::TextInputChanged);

    let submit = button("Send")
        .on_press(Message::SendPressed)
        .style(theme::Button::Positive);

    let request_bar = row![
        horizontal_space(10),
        method,
        row![text_box].width(Length::Fill),
        horizontal_space(10),
        submit,
        horizontal_space(10),
    ];

    let req_body = text(&sel.response)
        .size(20)
        .style(theme::Text::Color(Color::from_rgb(
            222.0 / 255.0,
            64.0 / 255.0,
            53.0 / 255.0,
        )));

    let scroller: iced::widget::scrollable::Scrollable<'_, Message, Renderer> =
        scrollable(column![req_body].width(Length::Fill))
            .vertical_scroll(Properties::new().width(5).scroller_width(5))
            .on_scroll(Message::Scrolled);

    let request_panel = column![
        vertical_space(10),
        request_bar.height(Length::FillPortion(1)),
        // vertical_space(5),
        scroller.height(Length::FillPortion(5)),
    ];

    let request_panel = Container::new(column![request_panel]).style(
        iced::theme::Container::Custom(Box::new(CustomContainerTheme::CustomBG(Color::from_rgb(
            33.0 / 255.0,
            33.0 / 255.0,
            33.0 / 255.0,
        )))),
    );

    return request_panel;
}

fn get_response_panel<'a>(sel: &BoltState) -> Container<'a, Message> {
    let response = text(&sel.response)
        .size(20)
        .style(theme::Text::Color(Color::from_rgb(
            222.0 / 255.0,
            64.0 / 255.0,
            53.0 / 255.0,
        )));

    let scroller: iced::widget::scrollable::Scrollable<'_, Message, Renderer> =
        scrollable(column![response].width(Length::Fill))
            .vertical_scroll(Properties::new().width(5).scroller_width(5))
            .on_scroll(Message::Scrolled);

    let response_panel = column![scroller];

    let response_panel = Container::new(column![response_panel]).style(
        iced::theme::Container::Custom(Box::new(CustomContainerTheme::CustomBG(Color::from_rgb(
            33.0 / 255.0,
            33.0 / 255.0,
            33.0 / 255.0,
        )))),
    );

    return response_panel;
}

fn get_editor<'a>(sel: &BoltState) -> Column<'a, Message> {
    let request_panel = get_request_panel(sel);

    // let request_panel2 = get_request_panel(sel);
    let response_panel = get_response_panel(sel);

    let editor = column![
        request_panel.height(Length::FillPortion(1)),
        horizontal_rule(1),
        response_panel.height(Length::FillPortion(1)),
    ];

    return editor;
}

fn get_console<'a>(sel: &BoltState) -> Container<'a, Message> {
    let name = row![text("Console").size(20),];

    let logs = row![text(&sel.response).size(20),];

    let scroller: iced::widget::scrollable::Scrollable<'_, Message, Renderer> =
        scrollable(column![logs].width(Length::Fill))
            .vertical_scroll(Properties::new().width(5).scroller_width(5))
            .on_scroll(Message::Scrolled);

    let console = container(column![name, scroller]);

    return console;
}

fn get_body<'a>(sel: &BoltState) -> Row<'a, Message> {
    let sidebar = get_sidebar();
    let editor = get_editor(sel);

    let body = row![sidebar, editor];

    return body;
}

pub fn get_view(sel: &BoltState) -> Element<Message> {
    let header = get_header();
    let body = get_body(sel);

    let console = get_console(sel);
    let final_view: Element<Message> = column![
        vertical_space(5),
        header,
        horizontal_rule(1),
        body.height(Length::FillPortion(7)),
        horizontal_rule(1),
        console.height(Length::FillPortion(1))
    ]
    .into();

    return Element::from(
        container(final_view)
            .width(Length::Fill)
            .height(Length::Fill),
    );
}

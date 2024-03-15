use iced::{
    widget::{button, Column},
    Element, Sandbox,
};

use super::message::Message;

pub struct SearchWindow {
    pub term_entered: String,
}

impl Sandbox for SearchWindow {
    type Message = Message;

    fn view(&self) -> Element<Self::Message> {
        Column::new()
            .align_items(iced::Alignment::Center)
            .push(button("start").on_press(Message::KeyPressed('a')))
            .push(iced::widget::text(self.term_entered.clone()))
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::KeyPressed(key) => self.term_entered.push(key),
        }
    }

    fn new() -> Self {
        Self {
            term_entered: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("SearchRS")
    }
}

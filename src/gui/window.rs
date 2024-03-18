use std::path::Path;

use iced::{
    futures::{executor, TryFutureExt},
    widget::{scrollable::Viewport, Column, Row, Scrollable, Text, TextInput},
    Alignment::Start,
    Application, Command, Element, Theme,
};

use crate::search;

use super::message::Message;

pub struct SearchWindow {
    pub term_entered: String,
    pub results: Vec<String>,
    pub scrollable: Option<Viewport>,
}

impl Application for SearchWindow {
    type Message = Message;
    type Executor = executor::ThreadPool;
    type Theme = Theme;
    type Flags = ();

    fn view(&self) -> Element<Self::Message> {
        let searchrow = Row::new().push(
            TextInput::new("Search...", &self.term_entered)
                .on_input(Message::SearchTermChange)
                .on_submit(Message::Search),
        );

        let mut results = Column::new();
        for result in &self.results {
            results = results.push(Text::new(result));
        }
        let scrollable = Scrollable::new(results).on_scroll(Message::Scroll);

        Column::new()
            .align_items(Start)
            .push(searchrow)
            .push(scrollable)
            .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SearchTermChange(search) => {
                self.term_entered = search;
                return Command::perform(
                    search::find::find(
                        Path::new(crate::OUTPUT_PATH).into(),
                        self.term_entered.clone(),
                    )
                    .map_err(|err| err.to_string()),
                    Message::SearchCompleted,
                );
            }
            Message::Search => {
                if self.term_entered.is_empty() {
                    return Command::none();
                }
                return Command::perform(
                    search::find::find(
                        Path::new(crate::OUTPUT_PATH).into(),
                        self.term_entered.clone(),
                    )
                    .map_err(|err| err.to_string()),
                    Message::SearchCompleted,
                );
            }
            Message::SearchCompleted(results) => match results {
                Ok(results) => {
                    self.results = results.iter().map(|result| result.join("/")).collect()
                }
                Err(err) => self.results = vec![err],
            },
            Message::Scroll(viewport) => self.scrollable = Some(viewport),
        }
        Command::none()
    }

    fn new(_flags: ()) -> (SearchWindow, iced::Command<Message>) {
        (
            Self {
                term_entered: String::new(),
                results: Vec::new(),
                scrollable: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SearchRS")
    }
}

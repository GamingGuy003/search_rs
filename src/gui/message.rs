use iced::{theme::Scrollable, widget::scrollable};

#[derive(Debug, Clone)]
pub enum Message {
    //Scroll(scrollable::State),
    Search,
    SearchCompleted(Result<Vec<Vec<String>>, String>),
    SearchTermChange(String),
}

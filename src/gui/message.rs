use iced::widget::scrollable::Viewport;

#[derive(Debug, Clone)]
pub enum Message {
    Scroll(Viewport),
    Search,
    SearchCompleted(Result<Vec<Vec<String>>, String>),
    SearchTermChange(String),
}

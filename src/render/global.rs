use crate::{app::app::GlobalItem, traits::RenderItem};
use tui::{
    backend::Backend,
    layout::{Rect, Alignment},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

impl RenderItem for GlobalItem {
    fn render_item<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            GlobalItem::SearchBar(q) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .title("Search YouTube")
                    .title_alignment(Alignment::Center);
                let paragraph = Paragraph::new(q.iter().collect::<String>()).block(block);
                frame.render_widget(paragraph, rect);
            }
            GlobalItem::MessageBar(m) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL);
                let paragraph =
                    Paragraph::new(m.clone().unwrap_or(String::from("All good :)"))).block(block);
                frame.render_widget(paragraph, rect);
            }
        }
    }
}

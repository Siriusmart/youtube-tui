use crate::{
    app::{app::App, pages::global::GlobalItem},
    traits::RenderItem,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    // style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

impl GlobalItem {
    pub fn render_item<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
        message: &Option<String>,
    ) {
        match self {
            GlobalItem::SearchBar(search) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if selected {
                        Color::LightBlue
                    } else if hover {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }))
                    .title("Search YouTube")
                    .title_alignment(Alignment::Center);
                let paragraph = Paragraph::new(search.clone()).block(block);
                frame.render_widget(paragraph, rect);
            }
            GlobalItem::MessageBar => {
                // let color = Color::LightYellow;
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if message.is_some() {
                        Color::LightRed
                    } else {
                        Color::Reset
                    }));
                let paragraph =
                    Paragraph::new(message.clone().unwrap_or(String::from("All good :)")))
                        .block(block);
                frame.render_widget(paragraph, rect);
            }
        }
    }
}

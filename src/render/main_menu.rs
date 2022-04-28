use crate::{
    app::pages::main_menu::{MainMenuItem, MainMenuSelector},
    traits::RenderItem,
};
use tui::{
    backend::Backend,
    layout::{Rect, Alignment},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

impl RenderItem for MainMenuItem {
    fn render_item<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            MainMenuItem::SeletorTab(selector) => {
                let text = match selector {
                    MainMenuSelector::Trending => "Trending",
                    MainMenuSelector::Popular => "Popular",
                    MainMenuSelector::History => "History",
                };

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL);
                let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Center);

                frame.render_widget(paragraph, rect);
            }
            MainMenuItem::VideoList(videos, index) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL);
                let paragraph = Paragraph::new("").block(block);

                frame.render_widget(paragraph, rect);
            }
            MainMenuItem::VideoDetails(video) => {
                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL);
                let paragraph = Paragraph::new("").block(block);

                frame.render_widget(paragraph, rect);
            }
        }
    }
}

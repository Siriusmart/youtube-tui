use crate::{
    app::{pages::main_menu::{MainMenuItem, MainMenuSelector}, app::App},
    traits::RenderItem,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout, Rect, Direction},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

impl RenderItem for MainMenuItem {
    fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        selected: bool,
        hover: bool,
    ) {
        let style = Style::default().fg(if selected {
            Color::LightBlue
        } else if hover {
            Color::LightRed
        } else {
            Color::Reset
        });

        match self {
            MainMenuItem::SeletorTab(selector) => {
                let text = match selector {
                    MainMenuSelector::Trending => "Trending",
                    MainMenuSelector::Popular => "Popular",
                    MainMenuSelector::History => "History",
                };

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(style);
                let paragraph = Paragraph::new(text)
                    .block(block)
                    .alignment(Alignment::Center);

                frame.render_widget(paragraph, rect);
            }
            MainMenuItem::VideoList(data) => {
                let chunks = Layout::default()
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                    .direction(Direction::Horizontal)
                    .split(rect);

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL)
                    .border_style(style);

                let inner = block.inner(chunks[0]);

                frame.render_widget(block, chunks[0]);

                let block = Block::default()
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL);
                frame.render_widget(block, chunks[1]);

                if let Some((videos, list, _)) = data {
                    list.area(inner);
                    frame.render_widget(list.clone(), inner);
                }
            }
        }
    }
}

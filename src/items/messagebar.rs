use crate::{
    config::{AppearanceConfig, MainConfig},
    global::structs::Message,
};
use tui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
};
use tui_additions::framework::FrameworkItem;

/// a message bar item, contains no fields because the message is taken from `data.global.Message`
#[derive(Clone, Copy, Default)]
pub struct MessageBar;

impl FrameworkItem for MessageBar {
    fn render(
        &mut self,
        frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>,
        framework: &mut tui_additions::framework::FrameworkClean,
        area: tui::layout::Rect,
        popup_render: bool,
        _info: tui_additions::framework::ItemInfo,
    ) {
        if popup_render {
            return;
        }

        let appearance = framework.data.global.get::<AppearanceConfig>().unwrap();
        let message = framework.data.global.get::<Message>().unwrap();

        // display with different border style according to type of message and config
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(appearance.borders)
            .border_style(Style::default().fg(match message {
                Message::None => appearance.colors.outline,
                Message::Success(_) => appearance.colors.message_success_outline,
                Message::Error(_) => appearance.colors.message_error_outline,
                Message::Message(_) => appearance.colors.message_outline,
            }));

        let paragraph = Paragraph::new(
            message.to_string(
                &framework
                    .data
                    .global
                    .get::<MainConfig>()
                    .unwrap()
                    .message_bar_default,
            ),
        )
        .block(block);

        frame.render_widget(paragraph, area);
    }

    fn selectable(&self) -> bool {
        false
    }
}

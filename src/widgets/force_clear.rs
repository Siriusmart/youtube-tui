use tui::{widgets::Widget, buffer::Buffer, layout::Rect, style::{Color, Style}};

pub struct ForceClear;

impl Widget for ForceClear {
    fn render(self, rect: Rect, buffer: &mut Buffer) {
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                buffer.get_mut(x, y).set_style(Style::default().bg(Color::DarkGray));
            }
        }
    }
}

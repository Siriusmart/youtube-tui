use serde::{Deserialize, Serialize};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::line::{
        HORIZONTAL, ROUNDED_BOTTOM_LEFT, ROUNDED_BOTTOM_RIGHT, ROUNDED_TOP_LEFT, ROUNDED_TOP_RIGHT,
        VERTICAL,
    },
    widgets::Widget,
};

#[derive(Debug, Clone)]
pub struct TextList {
    pub items: Vec<String>,
    pub selected: usize,
    pub scroll: usize,
    pub area: Option<Rect>,
    pub style: Style,
    pub selected_style: Style,
}

impl Widget for TextList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.items.len() == 0 || area.height < 3 {
            return;
        }

        let width_minus_2 = (area.width - 2) as usize;
        let width_minus_5 = (area.width - 5) as usize;
        let mut y = area.y;

        buf.set_style(area, self.style);

        for (i, mut item) in self
            .items
            .into_iter()
            .enumerate()
            .skip(self.scroll)
            .take(area.height as usize - 2)
        {
            let mut len = item.chars().count();
            if len > width_minus_2 {
                for _ in width_minus_5..len {
                    item.pop();
                }

                item.push_str("...");
            }

            len = item.chars().count();

            if i == self.selected {
                let horizontal = HORIZONTAL.to_string().repeat(width_minus_2);

                let lines = format!(
                    "{}{}{}\n{}{}{}{}\n{}{}{}",
                    ROUNDED_TOP_LEFT,
                    horizontal,
                    ROUNDED_TOP_RIGHT,
                    VERTICAL,
                    item,
                    " ".repeat(area.width as usize - 2 - len),
                    VERTICAL,
                    ROUNDED_BOTTOM_LEFT,
                    horizontal,
                    ROUNDED_BOTTOM_RIGHT,
                );

                let mut rect = Rect {
                    x: area.x,
                    y: y,
                    width: area.width,
                    height: 1,
                };
                buf.set_style(rect, self.selected_style);

                rect = Rect {
                    x: area.x,
                    y: y + 1,
                    width: 1,
                    height: 1,
                };
                buf.set_style(rect, self.selected_style);

                rect = Rect {
                    x: area.x + area.width - 1,
                    y: y + 1,
                    width: 1,
                    height: 1,
                };
                buf.set_style(rect, self.selected_style);

                rect = Rect {
                    x: area.x,
                    y: y + 2,
                    width: area.width,
                    height: 1,
                };
                buf.set_style(rect, self.selected_style);

                for line in lines.lines() {
                    buf.set_string(area.x, y, line, Style::default());

                    y += 1;
                }
            } else {
                buf.set_string(area.x + 1, y, item, Style::default());

                y += 1;
            }
        }
    }
}

impl Default for TextList {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected: 0,
            scroll: 0,
            area: None,
            style: Style::default(),
            selected_style: Style::default().fg(Color::Yellow),
        }
    }
}

impl TextList {
    pub fn area(&mut self, area: Rect) {
        self.area = Some(area);
        self.update_scroll();
    }

    pub fn items(&mut self, items: Vec<String>) {
        self.items = items;
    }

    pub fn down(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
        self.update_scroll()
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
        self.update_scroll();
    }

    pub fn first(&mut self) {
        self.selected = 0;
        self.update_scroll();
    }

    pub fn last(&mut self) {
        self.selected = self.items.len() - 1;
        self.update_scroll();
    }

    pub fn update_scroll(&mut self) {
        if self.selected < self.scroll {
            self.scroll = self.selected;
        }

        if let Some(area) = self.area {
            if self.selected > self.scroll + area.height as usize - 3 {
                self.scroll = self.selected + 3 - area.height as usize;
            }
        }
    }

    pub fn style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn selected_style(&mut self, style: Style) {
        self.selected_style = style;
    }
}

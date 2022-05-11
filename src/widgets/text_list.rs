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
        if self.items.len() == 0 || area.height < 3{
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
            if item.len() > width_minus_2 {
                item = format!("{}...", &item[0..width_minus_5]);
            }

            if i == self.selected {
                let horizontal = HORIZONTAL.to_string().repeat(width_minus_2);

                let lines = format!(
                    "{}{}{}\n{}{}{}{}\n{}{}{}",
                    ROUNDED_TOP_LEFT,
                    horizontal,
                    ROUNDED_TOP_RIGHT,
                    VERTICAL,
                    item,
                    " ".repeat(area.width as usize - 2 - item.len()),
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
                    for (x, c) in (0..area.width).zip(line.chars()) {
                        buf.get_mut(area.x + x, y).set_char(c);
                    }
                    y += 1;
                }
            } else {
                for (x, c) in (0..area.width).zip(item.chars()) {
                    buf.get_mut(area.x + x + 1, y).set_char(c);
                }
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
    }

    pub fn items(&mut self, items: Vec<String>) {
        self.items = items;
    }

    pub fn down(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;

            if let Some(area) = self.area {
                if self.selected > self.scroll + area.height as usize - 3 {
                    //panic!("Selected");
                    self.scroll = self.selected + 3 - area.height as usize;
                }
            }
        }
    }

    pub fn up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll {
                self.scroll = self.selected;
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

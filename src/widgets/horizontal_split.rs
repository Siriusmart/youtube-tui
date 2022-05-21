use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::line::{
        HORIZONTAL, HORIZONTAL_DOWN, HORIZONTAL_UP, ROUNDED_BOTTOM_LEFT, ROUNDED_BOTTOM_RIGHT,
        ROUNDED_TOP_LEFT, ROUNDED_TOP_RIGHT, VERTICAL,
    },
    widgets::Widget,
};

#[derive(Debug, Clone)]
pub struct HorizontalSplit {
    pub percentages: Vec<usize>,
    pub border_style: Style,
}

impl Widget for HorizontalSplit {
    fn render(self, rect: Rect, buffer: &mut Buffer) {
        let mut width_left = rect.width as usize;

        let widths = self
            .percentages
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if i == self.percentages.len() - 1 {
                    return width_left;
                }
                let width = p * rect.width as usize / 100;
                width_left -= width;
                width
            })
            .collect::<Vec<_>>();

        let mut top = String::new();
        let mut mid = String::new();
        let mut bottom = String::new();

        buffer.set_style(Rect { height: 1, ..rect }, self.border_style);
        buffer.set_style(
            Rect {
                height: 1,
                y: rect.y + rect.height - 1,
                ..rect
            },
            self.border_style,
        );

        buffer.set_style(
            Rect {
                x: rect.x + rect.width - 1,
                y: rect.y + 1,
                width: 1,
                height: rect.height - 2,
            },
            self.border_style,
        );

        let mut x = rect.x;

        for (index, width) in widths.iter().enumerate() {
            buffer.set_style(
                Rect {
                    x,
                    y: rect.y + 1,
                    width: 1,
                    height: rect.height - 2,
                },
                self.border_style,
            );

            x += *width as u16;

            if index == 0 {
                top.push_str(ROUNDED_TOP_LEFT);
                bottom.push_str(ROUNDED_BOTTOM_LEFT);
            } else {
                top.push_str(HORIZONTAL_DOWN);
                bottom.push_str(HORIZONTAL_UP);
            }

            mid.push_str(VERTICAL);

            if index == widths.len() - 1 {
                top.push_str(&HORIZONTAL.repeat(*width - 2));
                mid.push_str(&" ".repeat(*width - 2));
                bottom.push_str(&HORIZONTAL.repeat(*width - 2));

                top.push_str(ROUNDED_TOP_RIGHT);
                mid.push_str(VERTICAL);
                bottom.push_str(ROUNDED_BOTTOM_RIGHT);
            } else {
                top.push_str(&HORIZONTAL.repeat(*width - 1));
                mid.push_str(&" ".repeat(*width - 1));
                bottom.push_str(&HORIZONTAL.repeat(*width - 1));
            }
        }

        buffer.set_string(rect.x, rect.y, &top, Style::default());

        for y in rect.y + 1..rect.y + rect.height - 1 {
            buffer.set_string(rect.x, y, &mid, Style::default());
        }

        buffer.set_string(rect.x, rect.y + rect.height - 1, &bottom, Style::default());
    }
}

impl Default for HorizontalSplit {
    fn default() -> Self {
        HorizontalSplit {
            percentages: vec![50, 50],
            border_style: Style::default(),
        }
    }
}

impl HorizontalSplit {
    pub fn inner(&self, rect: Rect) -> Vec<Rect> {
        let mut width_left = rect.width as usize;

        let widths = self
            .percentages
            .iter()
            .enumerate()
            .map(|(i, p)| {
                if i == self.percentages.len() - 1 {
                    return width_left;
                }
                let width = p * rect.width as usize / 100;
                width_left -= width;
                width
            })
            .collect::<Vec<_>>();

        let mut x = rect.x as usize + 1;

        widths
            .iter()
            .enumerate()
            .map(|(i, width)| {
                let cell_rect = Rect {
                    x: x as u16,
                    y: rect.y + 1,
                    width: *width as u16 - if i == widths.len() - 1 { 2 } else { 1 },
                    height: rect.height - 2,
                };
                x += width;
                cell_rect
            })
            .collect::<Vec<_>>()
    }

    pub fn percentages(self, percentages: Vec<usize>) -> Self {
        Self {
            percentages,
            ..self
        }
    }

    pub fn border_style(self, border_style: Style) -> Self {
        Self {
            border_style,
            ..self
        }
    }
}

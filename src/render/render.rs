use crate::{
    app::app::{App, Item},
    traits::RenderItem,
};
use std::collections::LinkedList;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Wrap},
    Frame, style::{Style, Color},
};

impl App {
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let size = frame.size();

        if size.width < 45 || size.height < 16 {
            let paragraph = Paragraph::new(format!("Window too small. Minimum size 45 x 12. Current size is {} x {}", size.width, size.height)).block(Block::default()).style(Style::default().fg(Color::Red)).wrap(Wrap{trim: true});
            frame.render_widget(paragraph, size);
            return;
        }

        let hover_selected = if let Some((x, y)) = self.hover {
            Some(self.selectable[y][x])
        } else {
            None
        };

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.state
                    .iter()
                    .map(|row| row.height)
                    .collect::<Vec<Constraint>>(),
            )
            .split(size);

        for (y, (row, row_chunk)) in self
            .state
            .iter_mut()
            .zip(vertical_chunks.clone().into_iter())
            .enumerate()
        {
            let mut constraints = LinkedList::new();
            let mut length = match row.centered {
                true => Some(0),
                false => None,
            };
            for item in row.items.iter() {
                constraints.push_back(item.constraint);
                if let Some(length_value) = length {
                    length = Some(match item.constraint {
                        Constraint::Length(l) | Constraint::Max(l) | Constraint::Min(l) => {
                            l + length_value
                        }
                        Constraint::Percentage(p) => length_value + size.width * p / 100,
                        _ => unreachable!(),
                    })
                }
            }

            if let Some(i) = length {
                let extra_constraint = Constraint::Length((size.width - i) / 2);
                constraints.push_front(extra_constraint);
            } else {
                constraints.push_front(Constraint::Length(0));
            }

            constraints.push_back(Constraint::Length(0));

            let mut chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.into_iter().collect::<Vec<Constraint>>())
                .split(row_chunk)
                .into_iter();

            frame.render_widget(Block::default(), chunks.next().unwrap());

            
            for (x, (chunk, item)) in chunks.zip(row.items.iter_mut().map(|i| &mut i.item)).enumerate() {
                let selected = self.selected == Some((x, y));

                let hover = hover_selected == Some((x, y));
                
                match item {
                    Item::Global(i) => {
                        i.render_item(frame, chunk, selected, hover, &self.message);
                    }
                    Item::MainMenu(i) => {
                        i.render_item(frame, chunk, selected, hover);
                    }
                }
            }
        }
    }
}

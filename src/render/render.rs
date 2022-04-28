use crate::{
    app::app::{App, Item},
    traits::RenderItem,
};
use std::collections::LinkedList;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType},
    Frame,
};

impl App {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>) {
        let size = frame.size();
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.state
                    .iter()
                    .map(|row| row.height)
                    .collect::<Vec<Constraint>>(),
            )
            .split(size);

        for (row, row_chunk) in self.state.iter().zip(vertical_chunks.clone().into_iter()) {
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

            for (chunk, item) in chunks.zip(row.items.iter().map(|i| &i.item)) {
                match item {
                    Item::Global(i) => {
                        i.render_item(frame, chunk);
                    }
                    Item::MainMenu(i) => {
                        i.render_item(frame, chunk);
                    }
                }
            }
        }
    }
}

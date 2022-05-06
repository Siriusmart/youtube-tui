use tui::{backend::Backend, layout::Rect, Frame};

use crate::app::app::App;

pub trait RenderItem {
    fn render_item<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect, selected: bool, hover: bool);
}
use tui::{backend::Backend, layout::Rect, Frame};

pub trait RenderItem {
    fn render_item<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect);
}

use std::error::Error;

use crate::{
    app::app::{App, AppNoState},
    structs::{Item, WatchHistory},
};
use crossterm::event::KeyEvent;
use tui::{backend::Backend, layout::Rect, Frame};

pub trait ItemTrait {
    fn key_input(&mut self, key: KeyEvent, app: App) -> (bool, App);
    fn render_item<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        rect: Rect,
        app: AppNoState,
        selected: bool,
        hover: bool,
        popup_focus: bool,
        popup_render: bool,
    ) -> (bool, AppNoState);
    fn select(&mut self, app: App) -> (App, bool);
    fn selectable(&self) -> bool;
    fn load_item(
        &self,
        app: &App,
        watch_history: &mut WatchHistory,
    ) -> Result<Item, Box<dyn Error>>;
    fn reset(&mut self) {}
}

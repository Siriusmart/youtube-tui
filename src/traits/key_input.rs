use crate::app::app::App;
use crossterm::event::KeyCode;

pub trait KeyInput {
    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App);
}

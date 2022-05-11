use crossterm::event::KeyCode;
use crate::app::app::App;

pub trait KeyInput {
    fn key_input(&mut self, key: KeyCode, app: App) -> (bool, App) ;
}
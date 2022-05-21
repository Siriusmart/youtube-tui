use crate::app::app::App;

pub trait SelectItem {
    fn select(&mut self, app: App) -> (App, bool);
    fn selectable(&self) -> bool;
}

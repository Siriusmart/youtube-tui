use std::error::Error;

use crate::app::app::App;

pub trait LoadItem {
    fn load_item(&self, app: &App) -> Result<Box<Self>, Box<dyn Error>>;
}
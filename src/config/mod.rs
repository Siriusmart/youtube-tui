//! Structs for config files, all impl ConfigTrait, config files are stored in `~./config`
mod main;
pub use main::*;
mod appearance;
pub use appearance::*;
mod search;
pub use search::*;
mod pages;
pub use pages::*;
mod keybindings;
pub use keybindings::*;

pub mod serde;

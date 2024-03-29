//! Structs for config files, all impl ConfigTrait, config files are stored in `~./config`
mod remap;
pub use remap::*;
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
mod commands;
pub use commands::*;
mod commandbindings;
pub use commandbindings::*;
mod commands_remap;
pub use commands_remap::*;

pub mod serde;

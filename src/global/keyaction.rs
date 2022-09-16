use serde::{Deserialize, Serialize};

// Each key + modifier is bind to a key in a HashMap<KeyCode, HashMap<u8, KeyAction>>
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Select,
    Deselect,
    Exit,
    Back,
    Reload,
}

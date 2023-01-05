use serde::{Deserialize, Serialize};

/// Part of the keybindings config
// Each key + modifier is bind to a key in a HashMap<KeyCode, HashMap<u8, KeyAction>>
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyAction {
    /// Moves cursor to the left direction, or to the first item
    MoveLeft,
    /// Moves cursor to the right direction, or to the last item
    MoveRight,
    /// Moves cursor up
    MoveUp,
    /// Moves cursor dowm
    MoveDown,
    /// select the current item
    Select,
    /// deselect the current item, reverts the cursor back to its hover state
    Deselect,
    /// exits/quits the entire app
    Exit,
    /// revert last history
    Back,
    /// removes all history
    ClearHistory,
    /// reverts back to first history (home page if it is not cleared)
    FirstHistory,
    /// reload the current page
    Reload,
    /// start command capture
    StartCommandCapture,
    /// paste text
    Paste,
}

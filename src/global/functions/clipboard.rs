use clipboard::{ClipboardContext, ClipboardProvider};

/// get the clipboard content
pub fn get_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap_or_default()
}

/// set the clipboard content, panics if failed
pub fn set_clipboard(s: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(s).unwrap();
}

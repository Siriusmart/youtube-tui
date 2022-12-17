use tui::layout::Rect;
use tui_additions::widgets::TextField;
use typemap::Key;

/// a struct for storing different info, currently only stores one info
#[derive(Clone)]
pub struct Status {
    pub popup_opened: bool,
    pub search_filter_opened: bool,
    /// to prevent rerendering the same image
    pub render_image: bool,
    /// the textfield for command capture
    pub command_capture: Option<TextField>,
    /// if true, exit in the next iteration
    pub exit: bool,
    /// stores the area of the previously rendered frame
    pub prev_frame: Option<Rect>,
}

impl Key for Status {
    type Value = Self;
}

impl Default for Status {
    fn default() -> Self {
        Self {
            popup_opened: false,
            search_filter_opened: false,
            render_image: true,
            command_capture: None,
            exit: false,
            prev_frame: None,
        }
    }
}

impl Status {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

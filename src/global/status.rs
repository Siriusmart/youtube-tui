use typemap::Key;

#[derive(Clone)]
pub struct Status {
    pub popup_opened: bool,
}

impl Key for Status {
    type Value = Self;
}

impl Default for Status {
    fn default() -> Self {
        Self {
            popup_opened: false,
        }
    }
}

impl Status {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

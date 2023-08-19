use typemap::Key;

/// Messages in the message bar
// is stored in `data.global`
#[derive(Clone, PartialEq, Eq)]
pub enum Message {
    Message(String),
    Error(String),
    Success(String),
    Mpv(String),
    None,
}

impl Key for Message {
    type Value = Self;
}

impl Message {
    // to_string is used by the message bar to convert Message to string
    pub fn to_string(&self, default: &str) -> String {
        match self {
            Self::Message(msg) | Self::Error(msg) | Self::Success(msg) | Self::Mpv(msg) => {
                msg.to_string()
            }
            Self::None => default.to_string(),
        }
    }

    pub fn is_none(&self) -> bool {
        self == &Self::None
    }
}

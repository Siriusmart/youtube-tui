use typemap::Key;

/// Messages in the message bar
// is stored in `data.global`
#[derive(Clone)]
pub enum Message {
    Message(String),
    Error(String),
    Success(String),
    None,
}

impl Key for Message {
    type Value = Self;
}

impl Message {
    // to_string is used by the message bar to convert Message to string
    pub fn to_string(&self, default: &str) -> String {
        match self {
            Self::Message(msg) | Self::Error(msg) | Self::Success(msg) => msg.to_string(),
            Self::None => default.to_string(),
        }
    }
}

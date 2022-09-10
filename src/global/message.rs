use typemap::Key;

// Messages in the message bar, the struct is global
#[derive(Clone)]
pub enum Message {
    Message(String),
    Error(String),
    None,
}

impl Key for Message {
    type Value = Self;
}

impl Message {
    // to_string is used by the message bar to convert Message to string
    pub fn to_string(&self, default: &str) -> String {
        match self {
            Message::Message(msg) => msg.to_string(),
            Message::Error(msg) => msg.to_string(),
            Message::None => default.to_string(),
        }
    }
}

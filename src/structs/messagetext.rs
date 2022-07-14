#[derive(Clone, PartialEq, Eq)]
pub enum MessageText {
    Command(String),
    Text(String),
    None,
}

impl ToString for MessageText {
    fn to_string(&self) -> String {
        match self {
            Self::Command(cmd) => format!(":{}", cmd),
            Self::Text(s) => s.clone(),
            Self::None => String::from("All good :)"),
        }
    }
}

impl MessageText {
    pub fn is_some(&self) -> bool{
        *self != Self::None
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suggestions {
    pub query: String,
    pub suggestions: Vec<String>,
}

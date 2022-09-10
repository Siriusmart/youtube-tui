use invidious::reqwest::blocking::Client;
use typemap::Key;

// Holds the invidious client
#[derive(Clone)]
pub struct InvidiousClient(pub Client);

impl InvidiousClient {
    pub fn new(instance: String) -> Self {
        Self(Client::new(instance))
    }
}

impl Key for InvidiousClient {
    type Value = Self;
}

use invidious::ClientSync;
use typemap::Key;

// used in `data.global`
/// Holds the invidious client
#[derive(Clone)]
pub struct InvidiousClient(pub ClientSync);

impl InvidiousClient {
    pub fn new(instance: String) -> Self {
        Self(ClientSync::default().instance(instance))
    }
}

impl Key for InvidiousClient {
    type Value = Self;
}

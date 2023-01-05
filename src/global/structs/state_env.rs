use typemap::Key;

/// holds all modified envs that belongs to the page
/// stores (key, value) pairs
#[derive(Default, Clone)]
pub struct StateEnvs(pub Vec<(String, String)>);

impl Key for StateEnvs {
    type Value = Self;
}

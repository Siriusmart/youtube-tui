use std::env;

/// removes all environment variables in a vector
pub fn clear_envs(modified: &mut Vec<(String, String)>) {
    modified.iter().for_each(|(key, _)| env::remove_var(key));
    modified.clear();
}

/// set all environment variables in an interator, while pushing changes into the modified vector
/// to keep track of changes
pub fn set_envs<T: Iterator<Item = (String, String)>>(
    envs: T,
    modified: &mut Vec<(String, String)>,
) {
    envs.for_each(|(key, value)| {
        if !modified.iter().any(|(k, _)| k == &key) {
            modified.push((key.clone(), value.clone()));
        }
        env::set_var(key, value);
    });
}

/// replace env placeholder in a string with the value of the env
pub fn apply_envs(mut s: String) -> String {
    env::vars().for_each(|(key, value)| s = s.replace(&format!("${{{key}}}"), &value));
    s
}

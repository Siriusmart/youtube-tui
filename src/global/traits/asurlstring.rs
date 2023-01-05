/// A single URL param for search filters
pub trait AsUrlString {
    const TAG: &'static str;
    fn as_url_string(&self) -> String
    where
        Self: ToString,
    {
        format!("{}={}", Self::TAG, self.to_string())
    }
}

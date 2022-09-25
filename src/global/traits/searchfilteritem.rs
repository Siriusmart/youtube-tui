/// trait for on single search filter
pub trait SearchFilterItem
where
    Self: Sized,
{
    // display name of the option
    const NAME: &'static str;

    fn option_name(&self) -> &'static str;
    fn ordering() -> Vec<&'static str>;
    fn selected_index(&self) -> usize;
    fn at_index(index: usize) -> Self;
}

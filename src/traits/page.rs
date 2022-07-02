use crate::app::config::LayoutConfig;

pub trait PageTrait {
    fn default() -> LayoutConfig;
}

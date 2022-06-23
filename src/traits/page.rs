use crate::structs::Row;

pub trait PageTrait {
    fn message() -> String {
        String::from("Loading page...")
    }

    fn min() -> (u16, u16);

    fn default() -> Vec<Row>;
}

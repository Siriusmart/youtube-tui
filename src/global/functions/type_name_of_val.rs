use std::any::type_name;

pub fn type_name_of_val<T: ?Sized>(_val: &T) -> &'static str {
    type_name::<T>()
}

pub fn insert_vec<T>(vec: &mut Vec<T>, to_insert: Vec<T>, mut index: usize) {
    to_insert.into_iter().for_each(|x| {
        vec.insert(index, x);
        index += 1;
    });
}

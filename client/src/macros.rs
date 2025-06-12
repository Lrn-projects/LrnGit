#[macro_export]
macro_rules! vec_of_path {
    ($($x:expr),*) => (vec![$(Path::new($x)),*]);
}

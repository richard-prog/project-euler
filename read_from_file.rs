pub fn read_from_file<T>(filename: &str) -> Vec<Vec<T>> {
    let s = std::fs::read_to_string(&filename);
    let mut v = Vec::Vec<T>::new();
    let mut capacity = 0;

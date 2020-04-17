pub fn new_array<T: Clone>() -> Vec<T> {
    Vec::new()
}

pub fn push_to<T: Clone>(array: &mut Vec<T>, element: &T) {
    array.push(element.clone());
}

pub fn element_at<T: Clone>(array: Vec<T>, index: usize) -> T {
    array[index].clone()
}

use crate::element::Element;


/// sort a vector of elements in ascending order with O(n * log(n)) maximum time complexity.
///
/// # Arguments
/// * `elements` - elements to sort (sorting will modify the vector).
pub fn sort(elements: &mut Vec<Element>) {
    // quicksort has a worst case of O(n**2). let's go with something like mergesort.
    if elements.len() <= 1 { 
        return;
    }
    let mid = elements.len() / 2;
    let mut left = elements[0..mid].to_vec();
    let mut right = elements[mid..].to_vec();
    sort(&mut left);
    sort(&mut right);
    for i in 0..elements.len() {
        if left.is_empty() {
            elements[i] = right.remove(0);
        } else if right.is_empty() {
            elements[i] = left.remove(0);
        } else if left[0] < right[0] {
            elements[i] = left.remove(0);
        } else {
            elements[i] = right.remove(0);
        }
    }
}

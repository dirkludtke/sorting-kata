use crate::element::Element;


/// sort a vector of elements in ascending order with O(n**2) time complexity.
///
/// # Arguments
/// * `elements` - elements to sort (sorting will modify the vector).
pub fn sort(elements: &mut Vec<Element>) {
    // even if you know faster algorithms, here, we want the simplest solution possible.
    // something like bubble sort or insertion sort is fine. maximum 6 lines of code.
    let n = elements.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if elements[j] > elements[j + 1] {
                elements.swap(j, j + 1);
            }
        }
    }
}

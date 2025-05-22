use crate::element::Element;


/// sort a vector of elements in ascending order with O(n * log(n)) time complexity.
///
/// # Arguments
/// * `elements` - elements to sort (sorting will modify the vector).
pub fn sort(elements: &mut Vec<Element>) {
    // this is a good time for quicksort. please use the first or last element as the pivot.
    // this is for didactic purposes as we want to explore where quicksort is not so quick.
    if elements.len() == 0 { // avoid that end gets negative
        return;
    }
    quicksort(elements, 0, elements.len() - 1);
}

fn quicksort(elements: &mut Vec<Element>, start: usize, end: usize) {
    if start >= end {
        return;
    }
    let mut index = start;
    for i in start..end {
        if elements[i] < elements[end] {
            elements.swap(i, index);
            index += 1;
        }
    }
    elements.swap(index, end);
    if start + 1 < index { // avoid that end gets negative
        quicksort(elements, start, index - 1);
    }
    quicksort(elements, index + 1, end);
}

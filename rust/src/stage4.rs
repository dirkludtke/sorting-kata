use crate::element::Element;

const SIZE: usize = 1 << 16; // 2^16 = 65536

/// sort a vector of elements in ascending order with O(n) maximum time complexity.
///
/// # Arguments
/// * `elements` - elements to sort (sorting will modify the vector).
pub fn sort(elements: &mut Vec<Element>) {
    // yeah. you can do it. there is a hint in the documentation of the Element struct.
    let mut sorted_elements = vec![Vec::new(); SIZE];
    loop {
        match elements.pop() {
            None => break,
            Some(element) => {
                sorted_elements[element.get_value() as usize].push(element);
            }
        }
    }
    for mut equal_elements in sorted_elements {
        loop {
            match equal_elements.pop() {
                None => break,
                Some(element) => {
                    elements.push(element);
                }
            }
        }
    }
}

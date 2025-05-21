use std::cell::RefCell;
use crate::counter::Counter;

/// a wrapper around integers which counts the number of comparison and value accesses.
/// - please note that numbers are of type u16, i.e. have to be in a range from 0 to 65535.
#[derive(Debug)]
pub struct Element<'a> {
    value: u16,
    counter: &'a RefCell<Counter>,
}

impl<'a> Element<'a> {

    pub fn new(counter: &'a RefCell<Counter>, value: u16) -> Self {
        Self { value, counter }
    }

    pub fn get_value(&self) -> u16 {
        self.counter.borrow_mut().increment();
        self.value
    }
}

impl<'a> std::fmt::Display for Element<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> PartialEq<Element<'_>> for Element<'a> {
    fn eq(&self, other: &Element<'_>) -> bool {
        self.value == other.get_value()
    }
}

impl<'a> Eq for Element<'a> {}

impl<'a> PartialOrd<Element<'_>> for Element<'a> {
    fn partial_cmp(&self, other: &Element<'_>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Element<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.get_value())
    }
}

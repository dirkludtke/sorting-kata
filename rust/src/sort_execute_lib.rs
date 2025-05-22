use std::cell::RefCell;
use std::io::{self, Write};
use std::str;
use std::panic::AssertUnwindSafe;

use crate::counter;
use crate::element;
use crate::testdata;

/// named sorting algorithm
pub struct SortFunction {
    pub name: &'static str,
    pub stage: fn(&mut Vec<element::Element>),
}

fn rs_vec_sort(data: &mut Vec<element::Element>) {
    data.sort();
}

pub const SORT_FUNCTIONS: [SortFunction; 5] = [
    SortFunction {
        name: "stage1",
        stage: crate::stage1::sort,
    },
    SortFunction {
        name: "stage2",
        stage: crate::stage2::sort,
    },
    SortFunction {
        name: "stage3",
        stage: crate::stage3::sort,
    },
    SortFunction {
        name: "stage4",
        stage: crate::stage4::sort,
    },
    SortFunction {
        name: "rs_vec",
        stage: rs_vec_sort,
    },
];

/// Result of a sorting function execution
pub struct SortResult {
    pub sorted_data: Vec<u16>,
    pub count: u32,
    pub count_string: String,
}

/// Executes a sorting function on a test data set.
///
/// # Arguments
///
/// * `sort_function` - the sorting function to test.
/// * `test_set` - the data to sort.
///
/// # Returns
///
/// array of the sorted data, comparison count and comparison count string.
///
/// # Example
///
/// ```
/// use sorting_kata::sort_execute_lib::execute;
/// use sorting_kata::element::Element;
///
/// fn example_sort(elements: &mut Vec<Element>) {
///     elements.sort();
/// }
///
/// let test_set = vec![3, 1, 2];
/// let result = execute(example_sort, test_set).unwrap();
/// println!("Sorted Data: {:?}", result.sorted_data);
/// println!("Operation Count: {}", result.count);
/// println!("Count String: {}", result.count_string);
/// ```
pub fn execute(
    sort_function: fn(&mut Vec<element::Element>),
    test_set: &Vec<u16>,
) -> Result<SortResult, String> {
    let ref_counter = RefCell::new(counter::Counter::new());
    let mut elements = Vec::<element::Element>::new();
    for &value in test_set {
        elements.push(element::Element::new(&ref_counter, value));
    }
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        sort_function(&mut elements);
    }));
    if let Err(panic_info) = result {
        if let Some(message) = panic_info.downcast_ref::<String>() {
            return Err(message.to_string());
        } else {
            return Err("Panic without message.".to_string());
        }
    }
    let count = ref_counter.borrow().count();
    let count_string = ref_counter.borrow().to_string();
    let mut sorted_data = Vec::<u16>::new();
    for element in elements {
        sorted_data.push(element.get_value());
    }
    Ok(SortResult {sorted_data, count, count_string})
}

/// load a test data file.
///
/// # Arguments@param path file path.
/// * `path` - the path to the test data file.
///
/// # Returns
/// * `Ok` - the test data structure.
/// * `Err` - an error message if the file could not be read or parsed.
pub fn load_data(path: &String) -> Result<Vec<Vec<u16>>, String> {
    match std::fs::read_to_string(path) {
        Ok(text) => {
            testdata::TestdataParser::parse(&text)
        }
        Err(err) => Err(err.to_string()),
    }
}

///     save the test (result) data to a file.
///
/// # Arguments
/// * `writer` - the writer to save the data to.
/// * `data` - data to save.
///
/// # Returns
/// * `Ok` - if the data was saved successfully.
/// * `Err` - an error message if the data could not be saved.
pub fn save_data<W: Write>(writer: &mut W, data: &[Vec<u16>]) -> io::Result<()> {
    writeln!(writer, "[")?;
    for (i, row) in data.iter().enumerate() {
        write!(writer, "  [")?;
        for (j, value) in row.iter().enumerate() {
            if j > 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "{}", value)?;
        }
        writeln!(writer, "]{}", if i < data.len() - 1 { "," } else { "" })?; // End the row
    }
    writeln!(writer, "]")?;
    Ok(())
}

/// retrieve a named sorting function.
///
/// # Arguments
/// * `name` - the name of the sorting function.
///
/// # Returns
/// * `Ok` - the sorting function.
/// * `Err` - an error message if it does not exit.
pub fn get_sort_function(name: &str) -> Result<fn(&mut Vec<element::Element>), String> {
    for sort_function in SORT_FUNCTIONS.iter() {
        if name == sort_function.name {
            return Ok(sort_function.stage);
        }
    }
    Err(format!("Unknown algorithm: {}", name))
}

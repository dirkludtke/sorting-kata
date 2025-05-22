use std::env;

use sorting_kata::element::Element;
use sorting_kata::sort_execute_lib::*;


/// test a sorting algorithm with all test data sets until it fails.
pub fn test(algorithm: &str) -> Result<(), String> {
    println!("testing {}...", algorithm);

    let sort_function= match get_sort_function(algorithm) {
        Ok(sort_function) => {
            sort_function
        }
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let test_dir = env::current_exe()
        .expect("Cannot find test data directory (1)")
        .parent()
        .expect("Cannot find test data directory (2)")
        .parent()
        .expect("Cannot find test data directory (3)")
        .parent()
        .expect("Cannot find test data directory (4)")
        .parent()
        .expect("Cannot find test data directory (5)")
        .join("testdata");
    println!("  test data directory: {}", test_dir.display());

    // go through all the test set stages
    for data_stage in 1..=5 {
        // load test data (input and expected output is in separate files)
        let input_name = format!("input_stage_{}.json", data_stage);
        let output_name = format!("output_stage_{}.json", data_stage);
        println!("  data sets of {}...", input_name);

        let input_path = test_dir.join(&input_name).to_str().unwrap().to_string();
        let output_path = test_dir.join(&output_name).to_str().unwrap().to_string();

        println!("  loading {} and {}...", input_name, output_name);
        let input_list = match load_data(&input_path) {
            Ok(input_list) => {
                input_list
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let output_list = match load_data(&output_path) {
            Ok(output_list) => {
                if output_list.len() != input_list.len() {
                    return Err(format!(
                        "Cannot test {} because output data is missing (or incorrect)",
                        input_name
                    ));
                }
                output_list
            }
            Err(_) => {
                return Err(format!(
                    "Cannot test {} because output data is missing (or incorrect)",
                    input_name
                ));
            }
        };
        match test_datasets(sort_function, &input_list, &output_list) {
            Ok(_) => {
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }
    Ok(())
}

pub fn test_datasets(
    sort: fn(&mut Vec<Element>),
    input_list: &[Vec<u16>],
    output_list: &[Vec<u16>],
) -> Result<(), String> {
    let mut fail_count: u32 = 0;
    let test_iterator = input_list.iter().zip(output_list.iter());
    for (test_id, (input_data, expected)) in test_iterator.enumerate() {
        match execute(sort, input_data) {
            Ok(result) => {
                println!(
                    "    case {} {} in {}",
                    test_id + 1, list_to_string(input_data), result.count_string
                );

                if result.sorted_data != *expected {
                    let output_str = list_to_string(&result.sorted_data);
                    let expected_str = list_to_string(expected);
                    println!(
                        "FAILED.\noutput   {} !=\nexpected {}",
                        output_str, expected_str
                    );
                    fail_count += 1;
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    if fail_count > 0 {
        let plural = if fail_count == 1 { "" } else { "s" };
        return Err(format!("{} test case{} failed.", fail_count, plural));
    }
    Ok(())
}

pub fn list_to_string(data: &[u16]) -> String {
    list_to_string_with_length(data, 4)
}

pub fn list_to_string_with_length(data: &[u16], max_length: isize) -> String {
    if max_length >= 0 {
        let max_length: usize = max_length.try_into().unwrap();
        if data.len() > max_length {
            let displayed: Vec<String> = data.iter().take(max_length).map(|n| n.to_string()).collect();
            return format!("[{}, ...] (length {})", displayed.join(", "), data.len());
        }
    }
    let displayed: Vec<String> = data.iter().map(|n| n.to_string()).collect();
    format!("[{}]", displayed.join(", "))
}

fn print_usage() {
    println!("\nTest a sorting algortithm with all test data sets until it fails");
    let binding = env::args().nth(0).unwrap();
    let exec_name = std::path::Path::new(&binding).file_name().unwrap().to_str().unwrap();
    let algorithms = SORT_FUNCTIONS.iter().map(|f| f.name).collect::<Vec<_>>().join(", ");
    println!("Usage: {} <algorithm>", exec_name);
    println!("- algorithm: one of {{{}}}", algorithms);
}

fn main() {
    std::panic::set_hook(Box::new(|_| {})); // don't panic
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage();
        std::process::exit(1);
    }
    let algorithm = &args[1];
    match test(algorithm) {
        Ok(_) => {
        }
        Err(err) => {
            eprintln!("\nError: {}", err.to_string());
            std::process::exit(1);
        }
    }
}

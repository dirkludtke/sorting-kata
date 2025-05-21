use std::env;

use sorting_kata::element::Element;
use sorting_kata::sort_execute_lib::*;


fn load_and_execute(
    sort_function: fn(&mut Vec<Element>),
    path: String,
) -> Result<(), String> {
    match load_data(&path) {
        Ok(test_sets) => {
            let mut output_data = Vec::<Vec<u16>>::new();
            for test_set in test_sets {
                match execute(sort_function, &test_set) {
                    Ok(result) => {
                        output_data.push(result.sorted_data);
                    }
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            }
            match save_data(&mut std::io::stdout(), &output_data) {
                Ok(_) => {},
                Err(err) => return Err(err.to_string()),
            }
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

fn print_usage() {
    println!("\nExecute a sort algorithm with some test data");
    let binding = env::args().nth(0).unwrap();
    let exec_name = std::path::Path::new(&binding).file_name().unwrap().to_str().unwrap();
    let algorithms = SORT_FUNCTIONS.iter().map(|f| f.name).collect::<Vec<_>>().join(", ");
    println!("Usage: {} <algorithm> <dataPath>", exec_name);
    println!("- algorithm: one of {{{}}}", algorithms);
    println!("- dataPath:  path of test data (json array of integer array test sets)");
}

fn main() {
    std::panic::set_hook(Box::new(|_| {})); // don't panic
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        std::process::exit(1);
    }
    let name = &args[1];
    let path = &args[2];
    match get_sort_function(name) {
        Ok(sort_function) => {
            match load_and_execute(sort_function, path.to_string()) {
                Ok(_) => {
                }
                Err(err) => {
                    eprintln!("\nError: {}", err.to_string());
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
            print_usage();
            std::process::exit(1);
        }
    }
}

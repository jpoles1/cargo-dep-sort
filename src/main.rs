extern crate regex;
use std::fs;
use regex::Regex;

fn load_toml(filename: &str) -> String {
    let toml_raw = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("Loaded in TOML data:\n{}", toml_raw);
    return toml_raw
}

fn split_toml(toml_string : String) -> (Vec<String>, Vec<String>) {
    let toml_header_re = Regex::new(r"(\[+[\w-]+\]+)").unwrap();
    let toml_headers : Vec<String> = toml_header_re.captures_iter(&toml_string)
        .map(|s| s[0].to_owned()).collect();
    let toml_tables : Vec<String> = toml_header_re.split(&toml_string)
        .map(|s| s.to_owned()).skip(1).collect();
    assert_eq!(toml_headers.len(), toml_tables.len());
    return (toml_headers, toml_tables)
}

fn check_table_sorted(toml_table:String) -> bool {
    let table_keys : Vec<&str> = toml_table.lines()
        .map(|s| s.split("=").next().unwrap_or("").trim())
        .filter(|s| s.len() > 0)
        .collect();
    let mut sorted_table_keys = table_keys.clone();
    sorted_table_keys.sort_unstable();
    return table_keys == sorted_table_keys
}

fn check_cargo_toml_sorted(toml_headers:Vec<String>, toml_tables:Vec<String>) -> Option<String> {
    let included_headers : Vec<&str> = vec![
        "[dependencies]", 
        "[dev-dependencies]",
        "[build-dependencies]",
        "[workspace.members]",
        "[workspace.exclude]",
    ];
    for (toml_header, toml_table) in toml_headers.iter().zip(toml_tables.iter()) {
        if included_headers.contains(&&**toml_header) {
            if !check_table_sorted(toml_table.to_string()) {
                return Some(toml_header.to_string())
            }
        }
    }
    return None
}

fn main() {
    let toml_string = load_toml("test_data/rustlings.toml");
    let (toml_headers, toml_tables) = split_toml(toml_string);
    let toml_sort_result = check_cargo_toml_sorted(toml_headers, toml_tables);
    if toml_sort_result.is_some() {
        eprintln!("Found unsorted TOML table: {}", toml_sort_result.unwrap());
        std::process::exit(65);
    }
    std::process::exit(0);
}


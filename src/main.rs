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
    let toml_parts : Vec<String> = toml_header_re.split(&toml_string)
        .map(|s| s.to_owned()).skip(1).collect();
    assert_eq!(toml_headers.len(), toml_parts.len());
    return (toml_headers, toml_parts)
}

fn check_cargo_toml_sorted(toml_headers:Vec<String>, toml_parts:Vec<String>) -> bool {
    let included_headers : Vec<&str> = vec![
        "[dependencies]", 
        "[dev-dependencies]",
        "[build-dependencies]",
        "[workspace.members]",
        "[workspace.exclude]",
    ];
    for (toml_header, toml_part) in toml_headers.iter().zip(toml_parts.iter()) {
        if included_headers.contains(&&**toml_header) {
            println!("{} - {}", toml_header, toml_part);
        }
    }
    return true
}

fn recreate_toml(toml_headers:Vec<String>, toml_parts:Vec<String>) -> String {
    for (toml_header, toml_part) in toml_headers.iter().zip(toml_parts.iter()) {
        println!("{} - {}", toml_header, toml_part);
    }
    return "test".to_string()
}

fn main() {
    let toml_string = load_toml("test_data/rustlings.toml");
    let (toml_headers, toml_parts) = split_toml(toml_string);
    println!("Headers: {:?}", toml_headers);
    check_cargo_toml_sorted(toml_headers, toml_parts);
}


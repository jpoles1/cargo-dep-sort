use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::{BufRead, BufWriter, Read, Write};

extern crate clap;
extern crate regex;
extern crate toml;

use clap::{App, Arg};
use regex::Regex;
use std::fs;

mod writer;
use writer::TomlWriter;

//Checks if filepath points to a .toml file
fn is_toml_filepath(filepath: &str) -> bool {
    let toml_filepath_re = Regex::new(r"^.*\.toml$").unwrap();
    return toml_filepath_re.is_match(filepath);
}

//Takes a file path and reads its contents in as plain text
fn load_file_contents(filepath: &str) -> String {
    let file_contents =
        fs::read_to_string(filepath).expect("ERROR: Something went wrong reading the file");
    return file_contents;
}

fn load_toml_file(toml_filepath: &str) -> String {
    //Check if a valid .toml filepath
    if !is_toml_filepath(toml_filepath) {
        eprintln!(
            "WARN: detected invalid path to .toml file:\n{}",
            toml_filepath
        )
    }
    //Fetch toml data
    load_file_contents(toml_filepath)
}

/// Returns the string if it needed sorting else None
/// sorts including version
fn check_table_sorted(toml_table: &toml::value::Table) -> Option<String> {
    let dep_table: Vec<String> = toml_table.iter()
        .map(|(key, val)| format!("{}={}", key, val)).collect();

    let mut sorted_table = dep_table.clone();
    sorted_table.sort_unstable();

    match dep_table == sorted_table {
        true => None,
        false => Some(sorted_table.join("\r\n")),
    }
}

// fn check_cargo_toml_sorted(table_header: String, toml_data: toml::Value) -> Option<String> {
//     if let Some(vals) = toml_data.get(table_header) {
//         if let Some(table) = vals.as_table() {
//             if let Some(new_deps_table) = check_table_sorted(table) {
//                 Some(new_deps_table);
//             }
//         }
//     }
//     None
// }

// fn read_to(sorted: &String, seek_to: &str, fd: &mut File) -> std::io::Result<String> {
//     let file = fd.try_clone()?;
//     let mut reader = BufReader::new(file);
//     let mut bytes: Vec<u8> = Vec::default();
//     // read to header
//     loop {
//         if reader.read(&mut bytes)? == 0 {
//             continue;
//         } else {
//             match bytes.windows(seek_to.len()).position(|win| win == seek_to.as_bytes()) {
//                 Some(pos) => {
//                     println!("found {}", seek_to);

//                     return write_from(pos + seek_to.len(), sorted)
//                 },
//                 None => continue,
//             }
//         }
//     }
//     Ok(0)
// }

// fn replace_unsorted<'s>(
//     pos: usize,
//     sorted: &String,
//     mut base: String,
// ) -> std::io::Result<String> {
//     //let (first, end) = base.split_at_mut(pos);
//     base.replace_range(pos..pos + sorted.len(), sorted);
//     Ok(base)
// }

// fn write_to_toml(
//     path: &str,
//     contents: &String,
//     sorted: &String,
//     seek_to: &str,
// ) -> std::io::Result<()> {
//     match contents
//         .as_bytes()
//         .windows(seek_to.len())
//         .position(|win| win == seek_to.as_bytes())
//     {
//         Some(pos) => {
//             println!("found {}", seek_to);

//             let sort_file = replace_unsorted(pos + seek_to.len(), sorted, contents.clone())?;
//             println!("sorted {}", sort_file);
//             let mut fd = File::open(path)?;
//             fd.write_all(sort_file.as_bytes())?;
//             fd.flush()
//         }
//         None => Ok(()),
//     }
// }

//TODO: implement unit/integration tests for all major functions
//TODO: write functions to write a properly sorted Cargo.toml file to disk

fn main() -> std::io::Result<()> {
    let included_headers: Vec<&str> = vec![
        "dependencies",
        "dev-dependencies",
        "build-dependencies",
        "workspace.members",
        "workspace.exclude",
    ];
    //Instantiate command line args through clap
    let matches = App::new("cargo-dep-sort")
        .author("Jordan Poles <jpdev.noreply@gmail.com>")
        .about("Helps ensure sorting of Cargo.toml file dependency list")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the Cargo.toml file to check")
                .required(true)
                .index(1),
        )
        .get_matches();
    //Get TOML data from file provided in cmd arg
    let toml_filepath = matches.value_of("INPUT").unwrap();
    let mut toml_raw = load_toml_file(toml_filepath);
    let toml_data: toml::Value = toml::de::from_str(&toml_raw)
        .expect("ERROR: Failed to read improperly formatted TOML file!");

    let mut tw = TomlWriter::new(toml_raw);
    //Check if appropriate tables in file are sorted
    for header in included_headers.iter() {
        if let Some(vals) = toml_data.get(header) {
            if let Some(table) = vals.as_table() {
                if let Some(new_deps_table) = check_table_sorted(table) {
                    tw.replace_dep(header, new_deps_table)?;
                }
            }
        }
    }
    tw.write_all_changes("./test_data/test.toml")
    //let toml_sort_result = check_cargo_toml_sorted(toml_data, );
    // if toml_sort_result.is_some() {
    //     eprintln!("FAIL: found unsorted Cargo.toml table: {}", toml_sort_result.unwrap());
    //     std::process::exit(65);
    // }
    // println!("PASS: the detected Cargo.toml file is properly sorted!");
    // std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_toml_filepath() {
        assert!(is_toml_filepath("/cargo.toml"));
        assert!(!is_toml_filepath("cargo.tomls"));
    }
}

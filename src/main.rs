use std::fs;
use std::fs::{ File, OpenOptions };
use std::env;
use std::path::{ PathBuf, };


use clap::{App, Arg};
use colored::{ Colorize };
use regex::Regex;

mod writer;
use writer::TomlWriter;
mod utils;

//Checks if filepath points to a .toml file
fn is_toml_filepath(filepath: &str) -> bool {
    let toml_filepath_re = Regex::new(r"^.*\.toml$").unwrap();
    return toml_filepath_re.is_match(filepath);
}

//Takes a file path and reads its contents in as plain text
fn load_file_contents(filepath: &str) -> String {
    let file_contents =
        fs::read_to_string(filepath)
        .expect(&format!("{} Something went wrong reading the file", "ERROR:".red()));
    return file_contents;
}

fn load_toml_file(toml_filepath: &str) -> Option<String> {
    //Check if a valid .toml filepath
    if !is_toml_filepath(toml_filepath) {
        eprintln!("{}", &format!("{} detected invalid path to .toml file:\n{}",
            "ERROR:".red(),
            toml_filepath
        ));
        return None
    }
    //Fetch toml data
    Some(load_file_contents(toml_filepath))
}

/// Returns the string if it needed sorting else None
/// sorts including version
fn check_table_sorted(toml_table: &toml::value::Table) -> Option<String> {
    //println!("{:#?}", toml_table);
    let mut s = String::default();
    for pair in toml_table.iter() {
        utils::expand_table(pair, &mut s);
    }

    let dep_table: Vec<&str> = s.split("\n").collect();
    let mut sorted_table = dep_table.clone();
    sorted_table.sort_unstable();

    match dep_table == sorted_table {
        true => None,
        // TODO: cross platform
        false => Some(sorted_table.join("\n")),
    }
}

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
        .arg(Arg::with_name("cwd")
                .short("c")
                .long("cwd")
                .value_name("CWD")
                .help("Sets cwd, must contain Cargo.toml")
                .takes_value(true))
        .get_matches();

    
    let cwd = env::current_dir()
        .expect(&format!("{} could not get cwd", "ERROR:".red()));

    // either default cwd or user selected
    let mut path = matches.value_of("cwd")
        .map_or(cwd, |s| PathBuf::from(s.to_owned()));
    match path.extension() {
        None => {
            path.push("Cargo.toml");
        },
        _ => {},
    }

    println!("{:#?}", path);

    let mut toml_raw = match load_toml_file(path.to_str().unwrap()) {
        Some(t) => t,
        None => std::process::exit(64),
    };

    let toml_data: toml::Value = toml::de::from_str(&toml_raw)
        .expect(&format!("{} Failed to read improperly formatted TOML file!", "ERROR:".red()));

    let mut tw = TomlWriter::new(&mut toml_raw);
    //Check if appropriate tables in file are sorted
    for header in included_headers.iter() {
        if let Some(vals) = toml_data.get(header) {
            if let Some(table) = vals.as_table() {
                if let Some(new_deps_table) = check_table_sorted(table) {
                    println!("{}", new_deps_table);
                    // TODO: cross platform
                    let full_header = format!("[{}]", header);
                    tw.replace_dep(&full_header, new_deps_table)?;
                }
            }
        }
    }

    tw.write_all_changes("./test_data/test.toml")?;

    println!("{} dependencies have been sorted!", "Success".bold().bright_green());
    Ok(())
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

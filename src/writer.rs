use std::fs::{ File, OpenOptions, };
use std::path::{ Path, PathBuf, };
use std::io::BufReader;
use std::io::{BufRead, BufWriter, Read, Write};

pub struct TomlWriter {
    contents: String,

}

impl TomlWriter {

    pub fn new(s: String) -> Self {
        TomlWriter {
            contents: s,
        }
    }

    pub fn swap_range(
        &mut self,
        pos: usize,
        sorted: String,
    ) {
        
        self.contents.replace_range(pos..(pos + sorted.len()-3), &sorted)
    }

    pub fn replace_dep(
        &mut self,
        seek_to: &str,
        sorted: String,
    ) -> std::io::Result<()> {
        match self.contents
            .as_bytes()
            .windows(seek_to.len())
            .position(|win| win == seek_to.as_bytes())
        {
            Some(pos) => {
                println!("found {}", seek_to);

                self.swap_range(pos + seek_to.len()+2, sorted);
                //println!("sorted {}", self.contents);
                Ok(())
                
            }
            None => Ok(()),
        }
    }

    pub fn write_all_changes(&self, path: &str) -> std::io::Result<()> {
        let p = PathBuf::from(path);
        println!("{:#?}", p);
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        println!("\r\nFINAL:  {}", self.contents);
        fd.write_all(self.contents.as_bytes())?;
        fd.flush()
    }
}
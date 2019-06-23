use std::fs::{ OpenOptions, };
use std::path::{ PathBuf, };
use std::io::{ Read, Write};


pub struct TomlWriter {
    contents: String,
    eo_table: Vec<u8>,
    header_brace: Vec<u8>,

}

impl TomlWriter {

    pub fn new(s: String) -> Self {
        TomlWriter {
            contents: s,
            //TODO
            eo_table: b"\n\n".to_vec(),
            header_brace: b"[".to_vec(),
        }
    }

    pub fn swap_range(
        &mut self,
        pos: usize,
        sorted: String,
    ) {
        let eo_table = String::from_utf8_lossy(&self.eo_table);
        let fmt_sort = format!("{}{}", sorted, "\n");
        let end = self.unsorted_len(pos).expect("unsorted failed");
        self.contents.replace_range(pos..end, &fmt_sort)
    }

    pub fn unsorted_len(&self, after_header: usize) -> Option<usize> {
        // TODO cross platform
        let mut window_buf = [0u8; 2];

        let mut curse = std::io::Cursor::new(self.contents.clone());
        curse.set_position(after_header as u64);

        let mut pos = after_header;
        loop {
            // read eol number of bytes
            curse.read_exact(&mut window_buf).expect("read failed");
            // make sure we dont split and not read the right bytes in a row
            pos += window_buf.len() - 1;
            curse.set_position((pos - 1) as u64);

            // if we find double eol or "[" return cursor position
            if (&window_buf == self.eo_table.as_slice()) | (&window_buf == self.header_brace.as_slice()) {
                return Some(pos)
            }
        }

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
                let cursor_pos = pos + seek_to.len();
                
                self.swap_range(cursor_pos, sorted);
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn write_all_changes(&self, path: &str) -> std::io::Result<()> {
        let p = PathBuf::from(path);
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        fd.write_all(self.contents.as_bytes())?;
        fd.flush()
    }
}
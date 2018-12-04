use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::input::{CursorMove, CursorMove::*};
use crate::terminal::Position;

pub struct Buffers
{
    buffers: Vec<Buffer>,
}

impl Buffers
{
    /*
    pub fn new() -> Self
    {
        Self { buffers: vec![] }
    }
    
    pub fn open(&mut self, path: &str) -> Result<(), std::io::Error>
    {
        self.buffers.push(Buffer::load(path)?);
        Ok(())
    }
    
    pub fn get(&self, path: &str) -> Option<&Buffer>
    {
        //for (i, buffer) in self.buffers.iter().enumerate() {
        //    if buffer.src_path == path {
        //        return self.buffers.get(i);
        //    }
        //}
        None
    }
    */
}

pub struct Buffer
{
    src_path: Option<PathBuf>,
    content: Vec<String>,
    cursor: Position,
}

impl Buffer
{
    pub fn load(path: &str) -> Result<Self, std::io::Error>
    {
        let mut pathbuf = PathBuf::new();
        let path = std::fs::canonicalize(path)?;
        pathbuf.push(path);

        log!(format!("normal path is {:?}", pathbuf));

        let content = if let Ok(content) = std::fs::read_to_string(pathbuf.as_path()) {
            content.split('\n').map(|r| String::from(r)).collect()
        } else {
            vec![]
        };

        let buffer = Self {
            cursor: (0, 0),
            src_path: Some(pathbuf),
            content,
        };
        Ok(buffer)
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), std::io::Error>
    {
        let mut buffer = vec![];
        File::open(path)
            .or(File::create(path))
            .and_then(|mut file| {
                for line in &self.content {
                    buffer.extend_from_slice(line.as_bytes());
                    buffer.extend_from_slice(b"\n");
                }
                file.write_all(&buffer)
            })
    }

    pub fn source_path(&self) -> &Option<PathBuf>
    {
        &self.src_path
    }

    pub fn insert(&mut self, c: char) -> Result<(), &'static str>
    {
        let (cx, cy) = self.get_cursor();
        if let Some(line) = self.content.get_mut(cy as usize) {
            line.insert(cx as usize, c);
            self.move_cursor(Relative(1, 0));
            Ok(())
        } else {
            Err("line not available")
        }
    }

    pub fn insert_newline(&mut self) -> Result<(), &'static str>
    {
        let (cx, cy) = self.get_cursor();
        log!("newline!");
        let line = self.content.get_mut(cy as usize);
        if line.is_none() {
            return Err("line not available");
        }
        let line = line.unwrap();

        {
            let (left, right) = {
                let (l, r) = line.split_at(cx as usize);
                (l.to_string(), r.to_string())
            };
            *line = left;
            self.content.insert((cy + 1) as usize, right);
        }

        self.move_cursor(Absolute(0, cy + 1));

        Ok(())
    }

    pub fn remove(&mut self) -> Result<(), &'static str>
    {
        let (cx, cy) = self.get_cursor();
        let before = cx - 1;

        if let Some(line) = self.content.get_mut(cy as usize) {
            // TODO: that is disgusting
            let len = line.len() as i64;
            if 0 <= before && before < len {
                line.remove(before as usize);
                self.move_cursor(Relative(-1, 0));
            }
            if before < 0 && len == 0 && 1 < self.content.len() {
                self.content.remove(cy as usize);
                self.move_cursor(EndOfRow(cy - 1));
            }
            Ok(())
        } else {
            Err("line not available")
        }
    }

    pub fn get_row_at(&self, line: usize) -> Option<&str>
    {
        self.content.get(line).and_then(|c| Some(c.as_ref()))
    }

    pub fn get_cursor(&self) -> Position
    {
        self.cursor
    }

    pub fn move_cursor(&mut self, mv: CursorMove)
    {
        let (x, y) = match mv {
            Absolute(x, y) => (x, y),
            EndOfRow(y) => (0, y),
            Relative(rx, ry) => {
                let (cx, cy) = self.get_cursor();
                (cx + rx, cy + ry)
            }
        };
        if let Some(line) = self.content.get(y as usize) {
            self.cursor.1 = y;
            let len = line.len() as i64;

            match mv {
                EndOfRow(_) => self.cursor.0 = len,
                _ => {
                    if 0 <= x {
                        self.cursor.0 = x;
                        if len < self.cursor.0 {
                            self.cursor.0 = len;
                        }
                    }
                }
            }
        }
    }
}

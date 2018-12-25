use std::path::PathBuf;

use crate::input::CursorMove;
use crate::terminal::Position;

use ::libloe::buffer as libloe;

pub struct Buffer
{
    lbuffer: libloe::Buffer,
}

impl Buffer
{
    pub fn create(path: &str) -> Result<Self, std::io::Error>
    {
        Ok(Self {
            lbuffer: libloe::create(path)?,
        })
    }

    pub fn load(path: &str) -> Result<Self, std::io::Error>
    {
        Ok(Self {
            lbuffer: libloe::load(path)?,
        })
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), std::io::Error>
    {
        libloe::write(&self.lbuffer, path)
    }

    pub fn source_path(&self) -> &Option<PathBuf>
    {
        &self.lbuffer.src_path
    }

    pub fn content_len(&self) -> usize
    {
        self.lbuffer.content.len()
    }

    pub fn inner_mut(&mut self) -> &mut libloe::Buffer
    {
        &mut self.lbuffer
    }

    pub fn insert(&mut self, c: char) -> Result<(), &'static str>
    {
        libloe::insert(&mut self.lbuffer, c)
    }

    pub fn insert_newline(&mut self) -> Result<(), &'static str>
    {
        libloe::insert_newline(&mut self.lbuffer)
    }

    pub fn remove(&mut self) -> Result<(), &'static str>
    {
        libloe::remove(&mut self.lbuffer)
    }

    pub fn get_row_at(&self, line: usize) -> Option<&str>
    {
        libloe::get_row_at(&self.lbuffer, line)
    }

    pub fn get_cursor(&self) -> Position
    {
        self.lbuffer.cursor
    }

    pub fn move_cursor(&mut self, mv: CursorMove)
    {
        libloe::move_cursor(&mut self.lbuffer, mv)
    }
}

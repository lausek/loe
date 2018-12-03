use crate::terminal::Position;

pub struct Buffers
{
    buffers: Vec<Buffer>,
}

impl Buffers
{
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
        for (i, buffer) in self.buffers.iter().enumerate() {
            if buffer.src_path == path {
                return self.buffers.get(i);
            }
        }
        None
    }
}

pub struct Buffer
{
    src_path: String,
    content: Vec<String>,
    cursor: Position,
}

impl Buffer
{
    pub fn load(path: &str) -> Result<Self, std::io::Error>
    {
        let content = std::fs::read_to_string(path)?;
        let buffer = Self {
            cursor: (0, 0),
            src_path: path.to_owned(),
            content: content.split('\n').map(|r| String::from(r)).collect(),
        };
        Ok(buffer)
    }

    pub fn at(&self, line: usize) -> Option<&str>
    {
        self.content.get(line).and_then(|c| Some(c.as_ref()))
    }

    pub fn cursor(&self) -> Position
    {
        self.cursor
    }

    pub fn move_cursor(&mut self, x: isize, y: isize)
    {
        self.cursor.0 += x;
        self.cursor.1 += y;
    }
}

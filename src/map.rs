use std::io::Read;

use cwinui::pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cell(pub u8);

pub struct Map {
    pub grid: Vec<Cell>,
    height: usize,
    width: usize,
}

impl Map {
    pub fn new(height: u32, width: u32) -> Self
    {
        Self {
            height: height as usize,
            width: width as usize,
            grid: Vec::with_capacity((height * width) as usize),
        }
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, std::io::Error>
    {
        let mut m = Self::new(0, 0);
        m.load(reader)?;

        Ok(m)
    }

    pub fn height(&self) -> usize
    {
        self.height
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn load<R: Read>(&mut self, reader: &mut R) -> Result <(), std::io::Error>
    {
        let mut u32_buf = [u8::default(); 4];

        reader.read_exact(&mut u32_buf)?;
        let height = u32::from_le_bytes(u32_buf) as usize;
        reader.read_exact(&mut u32_buf)?;
        let width = u32::from_le_bytes(u32_buf) as usize;

        self.height = height;
        self.width = width;

        reader.read_exact(&mut <[u8; 16]>::default())?;

        let mut buf = vec![u8::default(); height * width];
        reader.read_exact(&mut buf)?;

        self.grid.resize(height * width, Cell::default());

        for (c, b) in std::iter::zip(&mut self.grid, &buf) {
            *c = Cell { 0: *b };
        }

        Ok(())
    }

    pub fn dump(&self)
    {
        for y in 0..self.height {
            for x in 0..self.width {
                eprint!("{}", self.grid[pos!(self.width, y, x)].0 as char);
            }
            eprintln!("");
        }
    }
}

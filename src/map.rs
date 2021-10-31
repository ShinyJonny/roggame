use byteorder::{LittleEndian, ReadBytesExt};

pub struct Map {
    grid: Vec<Vec<u8>>,
    start_y: u32,
    start_x: u32,
    fin_y: u32,
    fin_x: u32,
}

impl Map {
    pub fn new() -> Self
    {
        Self {
            grid: Vec::new(),
            start_y: 0,
            start_x: 0,
            fin_y: 0,
            fin_x: 0,
        }
    }

    pub fn from_file(m_path: &str) -> Result<Self, std::io::Error>
    {
        let mut m = Self::new();
        m.load(m_path)?;

        Ok(m)
    }

    pub fn size(&self) -> (u32, u32)
    {
        if self.grid.len() == 0
        {
            (0, 0)
        }
        else
        {
            (self.grid.len() as u32, self.grid[0].len() as u32)
        }
    }

    pub fn dump_grid(&self)
    {
        for row in &self.grid
        {
            for col in row
            {
                print!("{:02?}", col);
            }
            println!();
        }
    }

    pub fn load(&mut self, file: &str) -> Result <(), std::io::Error>
    {
        let mut f = std::io::BufReader::new(std::fs::File::open(file)?);

        let max_y       = f.read_u32::<LittleEndian>()? as usize;
        let max_x       = f.read_u32::<LittleEndian>()? as usize;
        self.start_y    = f.read_u32::<LittleEndian>()?;
        self.start_x    = f.read_u32::<LittleEndian>()?;
        self.fin_y      = f.read_u32::<LittleEndian>()?;
        self.fin_x      = f.read_u32::<LittleEndian>()?;

        self.grid = Vec::with_capacity(max_y);

        for row in 0..max_y
        {
            self.grid.push(Vec::with_capacity(max_x));

            for _col in 0..max_x
            {
                self.grid[row].push(f.read_u8()?);
            }
        }

        Ok(())
    }
}

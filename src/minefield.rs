extern crate rand;

use std::fmt;

#[derive(Clone)]
enum SquareType {
    Mine,
    Field(u8),
}

#[derive(Clone)]
pub struct Square {
    flagged: bool,
    revealed: bool,
    square_type: SquareType,
}

impl Square {
    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

    pub fn is_flagged(&self) -> bool {
        !self.revealed && self.flagged
    }

    pub fn is_mine(&self) -> bool {
        match self.square_type {
            SquareType::Mine => true,
            _ => false,
        }
    }

    pub fn get_number(&self) -> Option<u8> {
        match self.square_type {
            SquareType::Field(n) => Some(n),
            _ => None,
        }
    }
}

pub struct Minefield {
    width: u16,
    height: u16,
    size: usize,
    revealed: u32,
    flagged: u32,
    mines: u32,
    grid: Vec<Square>,
}

impl Minefield {
    pub fn create(width: u16, height: u16, mines: u32) -> Result<Minefield, &'static str> {
        let gridsize = width as usize * height as usize;

        if mines as usize > gridsize - 1 {
            return Err("Too many mines!");
        }

        let mut field = Minefield {
            width: width,
            height: height,
            size: gridsize,
            revealed: 0,
            flagged: 0,
            mines: mines,
            grid: Vec::with_capacity(gridsize),
        };

        field.grid.resize(gridsize, Square {flagged: false, revealed: false, square_type: SquareType::Field(0) });

        // Randomly allocate mines
        for i in 0..mines {
            let position: usize = rand::random();
            let mut position = position % (gridsize - i as usize);

            for m in &mut field.grid {
                if let SquareType::Mine = m.square_type {
                    continue;
                }

                if position == 0 {
                    m.square_type = SquareType::Mine;
                    break;
                } else {
                    position -= 1;
                }
            }
        }

        // Fill in field sizes
        fn inc(obj: &mut Square) {
            if let SquareType::Field(ref mut v) = obj.square_type { *v += 1; };
        }

        for y in 0..height {
            for x in 0..width {
                {
                    let square = field.get_square(x, y).unwrap();
                    if let SquareType::Field(_) = square.square_type {
                        continue;
                    }
                }

                let left = x == 0;
                let right = x == width - 1;
                let top = y == 0;
                let bottom = y == height - 1;

                if !top {
                    inc(field.get_square_mut(x, y - 1).unwrap());
                    if !left { inc(field.get_square_mut(x - 1, y - 1).unwrap()); }
                    if !right { inc(field.get_square_mut(x + 1, y - 1).unwrap()); }
                }
                if !bottom {
                    inc(field.get_square_mut(x, y + 1).unwrap());
                    if !left { inc(field.get_square_mut(x - 1, y + 1).unwrap()); }
                    if !right { inc(field.get_square_mut(x + 1, y + 1).unwrap()); }
                }
                if !left { inc(field.get_square_mut(x - 1, y).unwrap()); }
                if !right { inc(field.get_square_mut(x + 1, y).unwrap()); }
            }
        }

        return Ok(field);
    }

    pub fn flag(&mut self, x: u16, y: u16) -> Result<&Square, &'static str> {
        let mut flagged: i8 = 0;

        {
            let s = self.get_square_mut(x, y)?;

            if !s.revealed {
                s.flagged = !s.flagged;

                if s.flagged {
                    flagged = 1;
                } else {
                    flagged = -1;
                }
            }
        }

        if flagged > 0 {
            self.flagged += 1;
        } else if flagged < 0 {
            self.flagged -= 1;
        }

        return Ok(self.get_square(x, y)?);
    }

    pub fn reveal(&mut self, x: u16, y: u16) -> Result<&Square, &'static str> {
        let mut ret = false;
        let mut revealed = false;
        let mut flagged = false;

        {
            let s = self.get_square_mut(x, y)?;

            if s.revealed {
                ret = true;
            } else if !s.is_mine() {
                revealed = true;

                if s.flagged {
                    flagged = true;
                }
            }

            s.flagged = false;
            s.revealed = true;

            match s.square_type {
                SquareType::Field(0) => {},
                _ => { ret = true; },
            };
        }

        if revealed {
            self.revealed += 1;
        }

        if flagged {
            self.flagged -= 1;
        }

        if ret {
            return Ok(self.get_square(x, y)?);
        }

        // Reveal around
        if y < self.height - 1 {
            self.reveal(x, y + 1)?;
            if x > 0 { self.reveal(x - 1, y + 1)?; }
            if x < self.width - 1 {self.reveal(x + 1, y + 1)?; }
        }
        if y > 0 {
            self.reveal(x, y - 1)?;
            if x > 0 { self.reveal(x - 1, y - 1)?; }
            if x < self.width - 1 {self.reveal(x + 1, y - 1)?; }
        }
        if x > 0 {self.reveal(x - 1, y)?; }
        if x < self.width - 1 {self.reveal(x + 1, y)?; }

        return Ok(self.get_square(x, y)?);
    }

    pub fn finish(&mut self) {
        for s in &mut self.grid {
            match s.square_type {
                SquareType::Mine => { s.revealed = true; },
                _ => {},
            };
        }
    }

    pub fn get_square_mut(&mut self, x: u16, y: u16) -> Result<&mut Square, &'static str> {
        if x > self.width {
            return Err("X is higher than width");
        } else if y > self.height {
            return Err("Y is higher than height");
        } else {
            return Ok(&mut self.grid[y as usize * self.width as usize + x as usize]);
        }
    }

    pub fn get_square(&self, x: u16, y: u16) -> Result<&Square, &'static str> {
        if x > self.width {
            return Err("X is higher than width");
        } else if y > self.height {
            return Err("Y is higher than height");
        } else {
            return Ok(&self.grid[y as usize * self.width as usize + x as usize]);
        }
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }

    pub fn get_height(&self) -> u16 {
        self.height
    }

    pub fn get_revealed(&self) -> u32 {
        self.revealed
    }

    pub fn get_flagged(&self) -> u32 {
        self.flagged
    }

    pub fn get_mines(&self) -> u32 {
        self.mines
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl fmt::Debug for Minefield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                match self.grid[y as usize * self.width as usize + x as usize].square_type {
                    SquareType::Mine => { out += "X" },
                    SquareType::Field(0) => { out += " " },
                    SquareType::Field(n) => { out += &*n.to_string() },
                };
            }
            out += "\n";
        }

        f.write_str(out.as_str())
    }
}

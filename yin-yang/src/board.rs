use std::{fmt, iter, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub enum Tile {
    None,
    Black,
    White,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        use Tile::*;
        write!(
            f,
            "{}",
            match self {
                // ·
                White => "██", // "■",
                Black => "░░",
                None => "::",
            }
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = BoardError;

    fn try_from(c: char) -> Result<Tile, Self::Error> {
        match c {
            'b' | 'B' => Ok(Tile::Black),
            'w' | 'W' => Ok(Tile::White),
            'u' => Ok(Tile::None),
            _ => Err(BoardError::ParseTile),
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Board {
    width: usize,
    grid: Vec<Tile>,
}

impl Board {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.grid.len()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "size: {} x {} | {}",
            self.width(),
            self.height(),
            self.size()
        )?;
        for (i, tile) in self.grid.iter().enumerate() {
            if i % self.width == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{:}", format!("{tile}"))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum BoardError {
    ParseWidth,
    ParseTile,
    Format,
}

impl std::fmt::Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BoardError is here!")
    }
}

impl std::error::Error for BoardError {}

impl From<ParseIntError> for BoardError {
    fn from(_err: ParseIntError) -> Self {
        Self::ParseWidth
    }
}

impl FromStr for Board {
    type Err = BoardError;

    /// parsing from the puzzles.com format
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, grid) = s.split_once(';').ok_or(BoardError::Format)?;
        let width: usize = width.parse()?;
        let grid: Vec<Tile> = grid
            .trim()
            .chars()
            .flat_map(|c| {
                iter::repeat_with(move || match c.clone() {
                    'W' => Tile::White,
                    'B' => Tile::Black,
                    _ => Tile::None,
                })
                .take(if c == 'W' || c == 'B' {
                    1
                } else {
                    c.to_digit(36).unwrap() as usize - 9
                })
            })
            .collect();
        assert_eq!(grid.len(), width * (grid.len() / width));
        Ok(Board { width, grid })
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub x: usize,
    pub y: usize,
    pub val: Tile
}

impl crate::puzzle::Player for Board {
    type Move = Move;

    fn play(&mut self, m: Move) -> bool {
        self.grid[m.y * self.width + m.x] = m.val;
        true
    }
}

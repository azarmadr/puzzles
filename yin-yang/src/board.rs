use std::{error::Error, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Tile {
    None,
    Black,
    White,
}

impl TryFrom<char> for Tile {
    type Error = BoardError;

    fn try_from(c: char) -> Result<Tile, Self::Error> {
        match c {
            'b' => Ok(Tile::Black),
            'w' => Ok(Tile::White),
            'u' => Ok(Tile::None),
            _   => Err(BoardError::ParseTile),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    width: usize,
    size: usize,
    grid: Vec<Tile>,
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

impl Error for BoardError {
    fn description(&self) -> &str {
        match self {
            BoardError::ParseWidth => "Unable to parse width",
            BoardError::ParseTile => "invalid tile char found in grid part of the format",
            BoardError::Format => "improper format of the string, expecting `;`",
        }
    }
}

impl From<ParseIntError> for BoardError {
    fn from(_err: ParseIntError) -> Self { Self::ParseWidth }
}

impl FromStr for Board {
    type Err = BoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, grid) = s.split_once(';').ok_or(BoardError::Format)?;
        let width: usize = width.parse()?;
        let grid: Vec<Tile> = grid.trim().chars().map(|c| {
            c.try_into()
        })
        .map(|r: Result<Tile, BoardError>| r.unwrap())
        .collect();
        Ok(Board{
            width,
            size: grid.len(),
            grid
        })
    }
}

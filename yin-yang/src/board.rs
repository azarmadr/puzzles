use {
    crate::puzzle::{player::Player, LemmaBasedGridSolver, PatternMatch},
    std::{fmt, iter, num::ParseIntError, str::FromStr},
};

#[derive(PartialEq, Debug, Clone)]
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
                // · // "■",
                White => "██",
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

#[derive(Debug, PartialEq)]
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

    #[inline]
    fn to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    #[inline]
    fn to_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    // fn _get_sub_grid(&self,(x, y): (usize, usize), w: usize, h: usize) -> &Board {
    //     let mut r = Board { width: w, grid: vec![]};
    //     for i in 0..w { for j in 0..h {
    //         r.grid.push(self[(x + i, y + j)]);
    //     }}
    //     &r.to_owned()
    // }
}

impl std::ops::Index<(usize, usize)> for Board {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[self.to_index(x, y)]
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
    pub val: Tile,
}

impl Move {
    fn add(&self, at: (usize, usize)) -> Self {
        Self {
            x: self.x + at.0,
            y: self.y + at.1,
            val: self.val.clone(),
        }
    }
}

impl FromStr for Move {
    type Err = BoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split_whitespace();
        let (x, y) = (s.next().unwrap().parse()?, s.next().unwrap().parse()?);
        let val = s
            .next()
            .unwrap()
            .chars()
            .next()
            .ok_or(BoardError::Format)?
            .try_into()?;
        Ok(Move { x, y, val })
    }
}

impl Player for Board {
    type Move = Move;

    fn play(&mut self, m: Move) -> bool {
        self.grid[m.y * self.width + m.x] = m.val;
        true
    }

    fn result(&self) -> Option<bool> {
        // TODO cteate checks
        None
    }
}
type Moves = Vec<Move>;

#[derive(Debug)]
pub struct PatternLemma {
    src: Board,
    pub solution: Moves,
}

impl FromStr for PatternLemma {
    type Err = BoardError;

    /// parsing from the puzzles.com format with moves attached
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (src, solution) = s.split_once('\n').ok_or(BoardError::Format)?;
        let src = src.parse()?;
        let solution = solution.lines().map(|l| l.parse().unwrap()).collect();

        Ok(Self { src, solution })
    }
}

impl LemmaBasedGridSolver<PatternLemma> for Board {
    fn apply(&mut self, l: &PatternLemma) -> bool {
        if let Some(x) = self.find_index(&l.src) {
            println!("Applying {}: at {x} ", l.src);
            for m in &l.solution {
                self.play(m.add(self.to_xy(x)));
            }
            return true;
        }
        false
    }
}

impl crate::PatternMatch for Board {
    fn find_index(&self, other: &Self) -> Option<usize> {
        let (width, height) = (other.width(), other.height());
        for x in 0..=(self.width - width) {
            for y in 0..=(self.height() - height) {
                let mut match_found = true;
                for i in 0..width {
                    for j in 0..height {
                        if self[(x + i, y + j)] != other[(i, j)] {
                            match_found = false;
                            break;
                        }
                    }
                    if !match_found {
                        break;
                    }
                }
                if match_found {
                    return Some(self.to_index(x, y));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_works() -> Result<(), BoardError> {
        let board: Board = "6;hBdWaBWbWBdWaBaWBWd".parse()?;
        let pat: Board = "2;aBWa".parse()?;
        let pat1: Board = "2;aBWa".parse()?;
        println!("{board}\n{pat}");
        assert_eq!(Some(7), board.find_index(&pat));

        assert_eq!(pat, pat1);
        println!("match_works");
        Ok(())
    }

    #[test]
    fn lemma_applies() -> Result<(), BoardError> {
        let mut board: Board = "6;hBdWBBWbWBdWaBaWBWd".parse()?;
        let lemma: PatternLemma = "2;BaBB\n1 0 W".parse()?;
        println!("{board}\n{}, {:?}", lemma.src, lemma.solution);
        board.apply(&lemma);
        println!("{board}");
        println!("lemma_applies");
        Ok(())
    }
}

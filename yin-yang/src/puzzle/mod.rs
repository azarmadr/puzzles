use {
    crate::board::{BoardError, BoardWithMoves, PatternLemma},
    std::{fmt, fs, process::exit, str::FromStr},
};

pub mod player;
pub mod rules;
pub use player::*;

#[derive(Debug)]
enum State {
    _Done,
    New,
    _Playing,
}

#[derive(Debug)]
pub struct Puzzle<T: Player> {
    pub board: T,
    moves: Vec<T::Move>,
    _task: String,
    _state: State,
}

impl<T: Player<Move = Move>, Move: Clone> Puzzle<T> {
    fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
    fn reset_to(&mut self, index: usize) -> Result<(), PlayerError> {
        todo!()
    }
    pub fn game<Lemma>(rules: &Vec<Lemma>, sol_file: &str) -> Result<(), PlayerError>
    where
        Self: fmt::Display + std::string::ToString + LemmaBasedGridSolver<Lemma> + FromStr,
        Move: fmt::Debug + FromStr,
    {
        let mut s: Puzzle<T> = fs::read_to_string(sol_file)?
            .parse()
            .map_err(|_| PlayerError)?;
        let mut i = 0;
        println!("{i}: {s}");
        s.apply_all(rules);
        println!("{i}: after applying rules.\n{s}");
        let mut current_move_count = vec![];

        let mut puzzle_out = vec![format!("{}", s.to_string(),)];
        if let Some(won) = s.board.result() {
            if won {
                println!("You completed the puzzle.\nCheckout your moves at `{sol_file}`!!!");
                puzzle_out.push(s.board.solution());
                std::fs::write(sol_file, puzzle_out.join("\n"))?;
                exit(0);
            } else {
                println!("{}", "You made a mistake somewhere")
            }
        }
        let mut play = |input: String| -> Result<(), PlayerError> {
            // log::trace!("{input}");
            println!("{input}");
            let mut res = input.split_whitespace();
            match res.next() {
                Some("s") => {
                    println!("Saving...");
                    std::fs::write(sol_file, puzzle_out.join("\n"))?;
                }
                Some("q") => {
                    println!("Exiting...");
                    std::fs::write(sol_file, puzzle_out.join("\n"))?;
                    exit(0)
                }
                Some("u") => {
                    if puzzle_out.len() < 2 {
                        return Ok(());
                    }
                    if let Some(m) = puzzle_out.pop() {
                        let current_move_count = current_move_count.pop().unwrap();
                        println!("undo: {m} current_move_count: {current_move_count}");
                        // s.reset_to(current_move_count)?;
                    }
                }
                Some("m") => {
                    for m in s.moves() {
                        println!("{m:?}");
                    }
                    println!("User Moves: {}", puzzle_out.join("\n"))
                }
                Some("c") => current_move_count.push(s.moves().len()),
                Some("cc") => current_move_count.clear(),
                Some("current_move_count") => println!("{:?}", current_move_count.pop()),
                Some("C") => println!("{current_move_count:?}"),
                Some("r") => s.reset_to(res.next().unwrap().parse()?)?,
                Some("p") => println!("Board:\n{s}"),
                Some(x) if x.starts_with(|c: char| c.is_digit(10)) => {
                    i += 1;
                    let r#move = input.parse().map_err(|_| PlayerError);
                    // log::info!("[{f}]({row}, {col})");
                    println!("{i}: {move:?}");
                    current_move_count.push(s.moves().len());
                    puzzle_out.push(input.clone().trim().to_string());
                    s.play(r#move?);
                    println!("Move {i}:\n{s}");
                    s.apply_all(rules);
                    println!("Solver {i}.\n{s}");
                    println!("{}", input.clone().trim());
                }
                x => {
                    // log::warn!("Unknown input = {x:?}\nContinuing...")
                    println!("Unknown input = {x:?}\nContinuing...")
                }
            }
            if let Some(won) = s.result() {
                if won {
                    println!("You completed the puzzle.\nCheckout your moves at `{sol_file}`!!!");
                    puzzle_out.push(s.solution());
                    std::fs::write(sol_file, puzzle_out.join("\n"))?;
                    exit(0);
                } else {
                    println!("{}", "You made a mistake somewhere")
                }
            }
            Ok(())
        };

        loop {
            let input = get_input("Your Move:")?;
            if play(input).is_err() {
                println!("Wrong Input")
            }
        }
    }
}

impl<T: Player<Move = Move> + FromStr, Move: FromStr> FromStr for Puzzle<T> {
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, BoardError> {
        let (s, moves) = s.split_once('\n').ok_or(BoardError::Format)?;
        let moves = moves
            .lines()
            .map(|l| l.parse().map_err(|_| BoardError::Format).unwrap())
            .collect();
        Ok(Self {
            board: s.parse().map_err(|_| BoardError::Format)?,
            moves,
            _task: s.to_string(),
            _state: State::New,
        })
    }
}

impl<T: Player<Move = crate::board::Move> + LemmaBasedGridSolver<PatternLemma>>
    LemmaBasedGridSolver<PatternLemma> for Puzzle<T>
{
    fn apply(&mut self, l: &PatternLemma) -> bool {
        let res = self.board.apply(l);
        // TODO need to append the applied moves
        // for m in &l.solution {
        //     self.moves.push(m.add(self.board.to_xy(x)));
        // }
        res
    }
}

impl<T: Player<Move = Move> + fmt::Display, Move: fmt::Debug> fmt::Display for Puzzle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)?;
        for m in &self.moves {
            write!(f, "\n{m:?}")?;
        }
        Ok(())
    }
}
impl<T: Player<Move = M>, M: Clone> Player for Puzzle<T> {
    type Move = M;

    fn play(&mut self, m: M) -> bool {
        self.moves.push(m.clone());
        self.board.play(m)
    }
    fn result(&self) -> Option<bool> {
        self.board.result()
    }
}

pub trait LemmaBasedGridSolver<Lemma> {
    fn apply(&mut self, l: &Lemma) -> bool;
    fn apply_all(&mut self, rules: &Vec<Lemma>) {
        for rule in rules {
            let _ = self.apply(&rule);
        }
    }
}

pub trait PatternMatch {
    fn find_index(&self, other: &Self) -> Option<usize>;
}

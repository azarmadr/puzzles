use {
    crate::board::{BoardError, PatternLemma},
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

impl<T, Move> Puzzle<T>
where
    T: Player<Move = Move> + fmt::Display,
    Move: Clone,
{
    fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
    fn _reset_to(&mut self, _index: usize) -> Result<(), PlayerError> {
        todo!()
    }
    pub fn game<Lemma>(rules: &Vec<Lemma>, sol_file: &str) -> Result<(), PlayerError>
    where
        Self: fmt::Display + std::string::ToString + LemmaBasedGridSolver<Lemma> + FromStr,
        Move: fmt::Debug + FromStr,
        Lemma: fmt::Debug + fmt::Display,
        <Puzzle<T> as FromStr>::Err: fmt::Display,
    {
        let sol_contents = fs::read_to_string(sol_file)?;
        let mut s: Puzzle<T> = sol_contents
            .parse()
            .map_err(|e: <Puzzle<T> as FromStr>::Err| {
                println!("failed to parse {}", e.to_string());
                PlayerError
            })?;
        let mut i = 0;
        println!("{i}: {s}");
        s.apply_all(rules);
        println!("{i}: after applying rules.\n{}", s.board);
        let mut current_move_count = vec![];

        let mut puzzle_out = vec![format!("{}", sol_contents.trim().to_string(),)];
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
                Some("SR") => rules.iter().for_each(|r| println!("rule: {r}")),
                // TODO reset is not yet supported
                // Some("r") => s.reset_to(res.next().unwrap().parse()?)?,
                Some("p") => println!("Board:\n{}", s.board),
                Some(x) if x.starts_with(|c: char| c.is_digit(10)) => {
                    i += 1;
                    puzzle_out.push(input.clone().trim().to_string());
                    current_move_count.push(s.moves().len());
                    println!("{i}: {input}");
                    println!("Move {i}:\n{s}");
                    for m in expand_range_glob(&input) {
                        let r#move = m.parse().map_err(|_| PlayerError);
                        // log::info!("[{f}]({row}, {col})");
                        s.play(&r#move?);
                    }
                    s.apply_all(rules);
                    println!("Solver {i}.\n{}", s.board);
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

impl<T, Move> FromStr for Puzzle<T>
where
    T: Player<Move = Move> + FromStr,
    Move: FromStr,
{
    type Err = BoardError;
    fn from_str(s: &str) -> Result<Self, BoardError> {
        let (s, moves) = s.split_once('\n').unwrap_or_else(|| (s, ""));
        let moves = moves.lines().filter_map(|l| l.parse().ok()).collect();
        let mut board: T = s.parse().map_err(|_| BoardError::Format)?;
        for m in &moves {
            board.play(m);
        }
        Ok(Self {
            board,
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

    fn play(&mut self, m: &M) -> bool {
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
        loop {
            let mut any_rule_applied = false;
            for rule in rules {
                any_rule_applied |= self.apply(&rule);
            }
            if !any_rule_applied {
                break;
            }
        }
    }
}

pub trait PatternMatch {
    fn find_index(&self, other: &Self) -> Option<usize>;
}

pub trait GridTransform {
    /// Rotates self 90 degrees clockwise
    fn rotate_right(&mut self) {
        self.transpose();
        self.flip_cols();
    }
    /// Flip (or mirrors) the rows.
    fn flip_rows(&mut self);
    /// Flip (or mirrors) the cols.
    fn flip_cols(&mut self);
    /// Transpose the grid so that columns become rows in new grid.
    fn transpose(&mut self);

    fn neg(&mut self);
}

fn expand_range_glob(input: &str) -> Vec<String> {
    if !input.contains(|c| c == ',' || c == '-') {
        return vec![input.to_string()];
    }

    let mut result = Vec::new();

    let glob = input
        .split_whitespace()
        .find(|s| s.contains(|c| c == ',' || c == '-'))
        .unwrap();

    for part in glob.split(',') {
        let range: Vec<&str> = part.split('-').collect();
        if range.len() == 2 {
            // Handle range
            let start = usize::from_str(range[0]).unwrap();
            let end = usize::from_str(range[1]).unwrap();
            result.extend(start..=end);
        } else {
            // Handle single number
            let number = usize::from_str(range[0]).unwrap();
            result.push(number);
        }
    }

    result
        .into_iter()
        .map(|x| input.replace(glob, &x.to_string()))
        .collect()
}

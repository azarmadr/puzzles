use std::str::FromStr;

#[derive(Debug)]
enum State {
    Done,
    New,
    Playing
}

#[derive(Debug)]
pub struct Puzzle<T, M> {
    pub board: T,
    moves: Vec<M>,
    task: String,
    state: State
}

impl<T: FromStr<Err = E>, M, E> Puzzle<T, M> {
    pub fn new(task: &str) -> Result<Self, E> {
        Ok(Self {
            board: task.parse()?,
            moves: vec![],
            task: task.to_string(),
            state: State::New,
        })
    }
}

pub trait Player {
    type Move;

    fn play(&mut self, r#move: Self::Move) -> bool;
}

impl<T: Player<Move = M>, M: Clone> Player for Puzzle<T, M> {
    type Move = M;

    fn play(&mut self, m: Self::Move) -> bool {
        self.moves.push(m.clone());
        self.board.play(m)
    }
}

pub trait LemmaBasedGridSolver {
    type Lemma;

    fn apply(&mut self, l: &Self::Lemma) -> bool;
}

pub trait PatternMatch {
    fn find_index(&self, other: &Self) -> Option<usize>;
}

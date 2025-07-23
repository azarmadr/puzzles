use {crate::PatternLemma, std::str::FromStr};

pub mod rules;

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

impl<T: FromStr<Err = E> + Player, E> Puzzle<T> {
    pub fn new(task: &str) -> Result<Self, E> {
        Ok(Self {
            board: task.parse()?,
            moves: vec![],
            _task: task.to_string(),
            _state: State::New,
        })
    }
}

impl<T: Player<Move = crate::board::Move> + LemmaBasedGridSolver<PatternLemma>>
    LemmaBasedGridSolver<PatternLemma> for Puzzle<T>
{
    fn apply(&mut self, l: &PatternLemma) -> bool {
        for m in &l.solution {
            self.moves.push(m.clone());
        }
        self.board.apply(l)
    }
}

pub trait Player {
    type Move;

    fn play(&mut self, r#move: Self::Move) -> bool;
}

impl<T: Player<Move = M>, M: Clone> Player for Puzzle<T> {
    type Move = M;

    fn play(&mut self, m: M) -> bool {
        self.moves.push(m.clone());
        self.board.play(m)
    }
}

pub trait LemmaBasedGridSolver<Lemma> {
    fn apply(&mut self, l: &Lemma) -> bool;
}

pub trait PatternMatch {
    fn find_index(&self, other: &Self) -> Option<usize>;
}

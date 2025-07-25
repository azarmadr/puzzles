use {
    crate::rules::Rules,
    std::{str,num::ParseIntError, error::Error, process::exit, io, fmt},
};

#[derive(Debug)]
pub struct PlayerError;

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerError is here!")
    }
}

impl From<ParseIntError> for PlayerError {
    fn from(e: ParseIntError) -> Self {
        println!("{e}");
        Self
    }
}

impl From<io::Error> for PlayerError {
    fn from(e: io::Error) -> Self {
        println!("{e}");
        Self
    }
}

impl Error for PlayerError {}

pub trait Player {
    type Move;

    fn play(&mut self, r#move: Self::Move) -> bool;
    fn result(&self) -> Option<bool>;
    fn solution(&self) -> String {todo!()}
    fn moves(&self) -> Vec<Self::Move> {todo!()}
    fn reset_to(&mut self, index: usize) -> Result<(), PlayerError> {todo!()}

    fn game<Lemma>(&mut self, rules: &Vec<Lemma>, sol_file: &str) -> Result<(), PlayerError>
    where
        Self: fmt::Display + std::string::ToString + super::LemmaBasedGridSolver<Lemma>,
        Self::Move: fmt::Debug + str::FromStr
    {
        let mut i = 0;
        println!("{i}: {self}");
        self.apply_all(rules);
        println!("{i}: after applying rules.\n{self}");
        let mut current_move_count = vec![];

        let mut puzzle_out = vec![format!("{}", self.to_string(),)];
        if let Some(won) = self.result() {
            if won {
                println!("You completed the puzzle.\nCheckout your moves at `{sol_file}`!!!");
                puzzle_out.push(self.solution());
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
                        // self.reset_to(current_move_count)?;
                    }
                }
                Some("m") => {
                    for m in self.moves() {
                        println!("{m:?}");
                    }
                    println!("User Moves: {}", puzzle_out.join("\n"))
                }
                Some("c") => current_move_count.push(self.moves().len()),
                Some("cc") => current_move_count.clear(),
                Some("current_move_count") => println!("{:?}", current_move_count.pop()),
                Some("C") => println!("{current_move_count:?}"),
                Some("r") => self.reset_to(res.next().unwrap().parse()?)?,
                Some("p") => println!("Board:\n{self}"),
                Some(x) if x.starts_with(|c: char| c.is_digit(10)) => {
                    i += 1;
                    let r#move = input.parse().map_err(|_| PlayerError);
                    // log::info!("[{f}]({row}, {col})");
                    println!("{move:?}");
                    current_move_count.push(self.moves().len());
                    puzzle_out.push(input.clone().trim().to_string());
                    self.play(r#move?);
                    println!("Move {i}:\n{self}");
                    self.apply_all(rules);
                    println!("Solver {i}.\n{self}");
                    println!("{}", input.clone().trim());
                }
                x => {
                    // log::warn!("Unknown input = {x:?}\nContinuing...")
                    println!("Unknown input = {x:?}\nContinuing...")
                }
            }
            if let Some(won) = self.result() {
                if won {
                    println!("You completed the puzzle.\nCheckout your moves at `{sol_file}`!!!");
                    puzzle_out.push(self.solution());
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
pub fn get_input(prompt: &str) -> Result<String, PlayerError> {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

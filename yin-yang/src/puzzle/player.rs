use std::{error::Error, io, num::ParseIntError, str};

#[derive(Debug)]
pub struct PlayerError;

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerError is here!")
    }
}

impl From<ParseIntError> for PlayerError {
    fn from(e: ParseIntError) -> Self {
        println!("parseint: {e}");
        Self
    }
}

impl From<io::Error> for PlayerError {
    fn from(e: io::Error) -> Self {
        println!("io: {e}");
        Self
    }
}

impl Error for PlayerError {}

pub trait Player {
    type Move;

    fn play(&mut self, r#move: &Self::Move) -> bool;
    fn result(&self) -> Option<bool>;
    fn solution(&self) -> String {
        todo!()
    }
}
pub fn get_input(prompt: &str) -> Result<String, PlayerError> {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

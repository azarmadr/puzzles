mod board;
use board::*;

fn main() -> Result<(), BoardError> {
    let board: Board = "Hello, world!".parse()?;

    println!("{board:?}");

    Ok(())
}

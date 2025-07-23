mod board;
use board::*;

mod puzzle;
use puzzle::*;

fn main() -> Result<(), BoardError> {
    let mut p: Puzzle<Board, Move> = Puzzle::new("6;hBdWaBWbWBdWaBaWBWd")?;
    let _ = p.play(Move {x: 0, y:4, val: Tile::Black});
    let _ = p.play(Move {x: 0, y:3, val: Tile::Black});
    let _ = p.play(Move {x: 0, y:2, val: Tile::Black});
    let _ = p.play(Move {x: 0, y:1, val: Tile::Black});
    for x in 2..6 {
        let _ = p.play(Move {x, y:5, val: Tile::White});
    }
    let _ = p.play(Move {x: 4, y: 4, val: Tile::Black});
    let _ = p.play(Move {x: 2, y: 4, val: Tile::Black});
    let _ = p.play(Move {x: 3, y: 3, val: Tile::White});
    let _ = p.play(Move {x: 4, y: 3, val: Tile::White});
    let _ = p.play(Move {x: 5, y: 3, val: Tile::White});
    let _ = p.play(Move {x: 2, y: 2, val: Tile::Black});
    for x in 0..6 {
        let _ = p.play(Move {x, y:0, val: Tile::Black});
    }
    for y in 1..3 {
        let _ = p.play(Move {x: 5, y, val: Tile::Black});
    }
    let _ = p.play(Move {x: 1, y: 1, val: Tile::White});
    let _ = p.play(Move {x: 3, y: 1, val: Tile::White});
    let _ = p.play(Move {x: 4, y: 1, val: Tile::White});
    println!("{p:?}\n{}", p.board);


    // let board: Board = "30;jBuBaBfBaBaWgWcWgBfBdBbWaWcWdBeBbBdWaWWbWaWaWg\
    //                     BBBbBBbBBbWiBaBcWdBcBWbBaBaBaBaBbBaBfBcBcWfBeBaBb\
    //                     BfBeWaWaWcWBWcBdBbWdBaBiBbBaBbBbBBaBBcBcWaWdBWcBg\
    //                     BbBdBfBaBbBaBbBBBBgBBaBaBaBWaBaWcBjBaBaWWWBdBaWaW\
    //                     BaBdBBaBiWaBWbWaWcBcBaWaBaBbBaWWbWBbBWcBaBfWBbBeW\
    //                     cBbBaWdBbBaWaBaBbBbWaBbBaWaBbBeBbWaBaBBaBbBgBdBdB\
    //                     aBcWWWWaBcBaWBaBaBeBeBeWaBbBcBcBaBcWaWaWaWaWWdBaB\
    //                     WBbWaBiWaWdWaWcBcWaWbWdWWWbBaBfBaBaBcWBpWeBcWdWdW\
    //                     WWWWeWaBcBaBbBbBhWbWbWcBbBaBWbBBbBcBaWcBaWaWBbBaB\
    //                     aBeBdBhWaWBaBaBbBbBaBhWWWWaWcBdBaBaBdBzf"
    //     .parse()?;
    // println!("{board}");

    Ok(())
}

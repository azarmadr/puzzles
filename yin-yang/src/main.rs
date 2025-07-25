mod board;
use board::*;

mod puzzle;
use puzzle::*;

fn main() -> Result<(), BoardError> {
    let r = rules::read_rules("assets/rules.yml");
    println!("{r:?}");
    let mut p: Puzzle<Board> = "6;hBdWaBWbWBdWaBaWBWd".parse()?;
    println!("start\n{}", p.board);
    let play = move |p: &mut Puzzle<Board>, x, y, val: Tile| {
        let _ = p.play(Move {
            x,
            y,
            val: val.clone(),
        });
        for rule in &r {
            p.apply(&rule);
        }
        println!("{}", p.board);
    };
    for y in 1..=4 {
        play(&mut p, 0, y, Tile::Black);
    }
    for x in 2..6 {
        play(&mut p, x, 5, Tile::White);
    }
    play(&mut p, 4, 4, Tile::Black);
    play(&mut p, 2, 4, Tile::Black);
    play(&mut p, 3, 3, Tile::White);
    play(&mut p, 2, 2, Tile::Black);
    play(&mut p, 3, 1, Tile::White);
    for x in 0..6 {
        play(&mut p, x, 0, Tile::Black);
    }
    play(&mut p, 4, 3, Tile::White);
    for y in 1..3 {
        play(&mut p, 5, y, Tile::Black);
    }
    play(&mut p, 1, 1, Tile::White);

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

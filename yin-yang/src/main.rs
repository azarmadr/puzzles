mod board;
use board::*;

fn main() -> Result<(), BoardError> {
    let board: Board = "6;hBdWaBWbWBdWaBaWBWd".parse()?;
    println!("{board}");

    let board: Board = "30;jBuBaBfBaBaWgWcWgBfBdBbWaWcWdBeBbBdWaWWbWaWaWg\
                        BBBbBBbBBbWiBaBcWdBcBWbBaBaBaBaBbBaBfBcBcWfBeBaBb\
                        BfBeWaWaWcWBWcBdBbWdBaBiBbBaBbBbBBaBBcBcWaWdBWcBg\
                        BbBdBfBaBbBaBbBBBBgBBaBaBaBWaBaWcBjBaBaWWWBdBaWaW\
                        BaBdBBaBiWaBWbWaWcBcBaWaBaBbBaWWbWBbBWcBaBfWBbBeW\
                        cBbBaWdBbBaWaBaBbBbWaBbBaWaBbBeBbWaBaBBaBbBgBdBdB\
                        aBcWWWWaBcBaWBaBaBeBeBeWaBbBcBcBaBcWaWaWaWaWWdBaB\
                        WBbWaBiWaWdWaWcBcWaWbWdWWWbBaBfBaBaBcWBpWeBcWdWdW\
                        WWWWeWaBcBaBbBbBhWbWbWcBbBaBWbBBbBcBaWcBaWaWBbBaB\
                        aBeBdBhWaWBaBaBbBbBaBhWWWWaWcBdBaBaBdBzf"
        .parse()?;
    println!("{board}");

    Ok(())
}

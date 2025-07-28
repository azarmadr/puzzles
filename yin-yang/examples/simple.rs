use {
    std::{env, fs},
    yin_yang::{
        board::Board,
        puzzle::{PlayerError, Puzzle, rules},
    },
};

fn main() -> Result<(), PlayerError> {
    println!("{:?}", env::args());
    let file = env::args().last().unwrap();
    let sol_file = if file.ends_with(".sol.txt") {
        file.clone()
    } else {
        file.clone().replace(".txt", ".sol.txt")
    };
    if fs::exists(&sol_file).is_ok_and(|x| !x) {
        fs::copy(file, &sol_file)?;
    }
    let rules = rules::read_rules("assets/rules.yml");
    Puzzle::<Board>::game(&rules, &sol_file)
}

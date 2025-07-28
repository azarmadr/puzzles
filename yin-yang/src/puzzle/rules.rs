// TODO forgo saphyr for a simpler string based file format
use {
    crate::{board::PatternLemma, puzzle::GridTransform},
    std::{
        fs::File,
        io::{prelude::*, BufReader},
    },
};

pub type Rules = Vec<PatternLemma>;
// one rule per document
pub fn read_rules(f: &str) -> Rules {
    let file = File::open(f).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    let mut rules = vec![];
    loop {
        let mut tmp_buf = String::new();
        let ret = reader.read_line(&mut tmp_buf);
        if ret.is_ok_and(|v| v == 0) {
            break;
        }
        if !tmp_buf.starts_with("---") {
            buffer += &tmp_buf;
        } else {
            if let Ok(rule) = buffer.trim().parse::<PatternLemma>() {
                let mut rule_tr = rule.clone();

                // TODO not all are being pushed
                // can be a method on the rule that can return all possible unique shapes
                rules.push(rule_tr.clone());
                rule_tr.neg();
                rules.push(rule_tr.clone());

                rule_tr.rotate_right();
                rules.push(rule_tr.clone());
                rule_tr.neg();
                rules.push(rule_tr.clone());

                rule_tr.rotate_right();
                rules.push(rule_tr.clone());
                rule_tr.neg();
                rules.push(rule_tr.clone());

                rule_tr.rotate_right();
                rules.push(rule_tr.clone());
                rule_tr.neg();
                rules.push(rule_tr);
            }
            buffer.clear();
        }
    }
    for rule in &rules {
        println!("{rule}");
    }
    rules
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

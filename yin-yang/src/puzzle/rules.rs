use {
    crate::PatternLemma,
    saphyr::{LoadableYamlNode, Yaml},
    std::{fs::File, io::prelude::*},
};

pub type Rules = Vec<PatternLemma>;
pub fn read_rules(f: &str) -> Rules {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut rules = vec![];
    for rule in Yaml::load_from_str(&contents).unwrap()[0].to_owned() {
        if let Some(rule) = rule.as_str() {
            if let Ok(rule) = rule.parse() {
                rules.push(rule)
            } else {
                // TODO add line file:line
                println!("WARN: failed to parse '{rule:?}' as PatternLemma from {f}")
            }
        }
    }
    rules
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Amount {
    pub tier: usize,
    pub special_abilities: usize,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
pub struct RollTable {
    pub name: Option<String>,
    #[builder(setter(each(name = "add_options")))]
    pub table: Vec<RollEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollEntry {
    pub start: usize,
    pub end: usize,
    pub entry: String,
}

pub fn load_option_table(input: &str) -> Vec<RollEntry> {
    let option_regex = Regex::new(r"(\d+)(-(\d+))? (.*)").unwrap();
    let mut out = vec![];
    for line in input.split('\n').map(|s| s.trim()) {
        if option_regex.is_match(line) {
            let captures = option_regex.captures(line).unwrap();
            let start = captures.get(1).unwrap().as_str().trim().parse().unwrap();
            let end = captures.get(3).map(|s| match s.as_str().trim() {
                "00" => 100,
                s => s.parse().unwrap(),
            }).unwrap_or(start);
            let effect = captures.get(4).unwrap().as_str().trim().into();
            out.push(RollEntry { start, end, entry: effect });
        }
    }
    out
}
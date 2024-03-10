use std::fs;

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::tables::*;

#[derive(Builder, Serialize, Deserialize, Debug)]
pub struct Cypher {
    pub name: String,
    pub tags: Vec<String>,
    pub level_dice: Option<String>,
    pub level_mod: usize,
    pub form: Option<String>,
    pub effect: String,
    pub options: Vec<RollTable>,
}

#[derive(PartialEq)]
enum CypherTableSM {
    Name,
    Option
}

pub fn load_cypher_tables(db_cyphers: &mut Vec<Cypher>) -> Vec<RollTable> {
    let mut out = vec![];
    let cyphers = unidecode(&fs::read_to_string("CypherTables.md").unwrap());
    let option_regex = Regex::new(r"(\d+)(-(\d+))? (.*)").unwrap();
    let mut current = RollTableBuilder::default();
    let mut current_kind = String::new();
    let mut phase = CypherTableSM::Name;
    for line in cyphers.split('\n').map(|s| s.trim()) {
        if phase == CypherTableSM::Name {
            current.name(Some(line.trim().into()));
            current_kind = line.trim().into();
            phase = CypherTableSM::Option;
        } else if option_regex.is_match(line) {
            let captures = option_regex.captures(line).unwrap();
            let start = captures.get(1).unwrap().as_str().parse().unwrap();
            let end = captures.get(3).map(|s| s.as_str().parse().unwrap()).unwrap_or(start);
            let effect : String = captures.get(4).unwrap().as_str().trim().into();
            db_cyphers.iter_mut().filter(|c| c.name == effect.to_ascii_uppercase()).next().expect(&effect).tags.push(current_kind.clone());
            current.add_options(RollEntry { start, end, entry: effect });
        } else if line.is_empty() {
            out.push(current.description(None).build().unwrap());
            current = RollTableBuilder::default();
            phase = CypherTableSM::Name;
        }
    }
    out
}

// Named regex: (?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Effect:\s*(?P<effect>.*)\s*(?P<options>OPTION TABLE\s*(?:(?:\d*)-?(?:\d*)?\s(?:.*)\s*)+)?
pub fn load_cyphers() -> Vec<Cypher> {
    let cyphers = unidecode(&fs::read_to_string("cyphers.md").unwrap());
    let cypher_regex = Regex::new(&format!(r"(?m)(?P<name>[^\n]*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*(?:Form:\s*(?P<form>.*)\s*)?Effect:\s*(?P<effect>[\s\w\W]*?){OPTION_TABLE_PATTERN}?(^\s*$)")).unwrap();
    let mut out = vec![];
    for capture in cypher_regex.captures_iter(&cyphers) {
        let cypher = CypherBuilder::default()
            .name(capture.name("name").map(|s| s.as_str().to_ascii_uppercase().trim().into()).unwrap())
            .level_dice(capture.name("dice").map(|s| s.as_str().trim().into()))
            .level_mod(capture.name("mod").and_then(|s| s.as_str().parse().ok()).unwrap_or(0))
            .form(capture.name("form").map(|s| s.as_str().to_ascii_uppercase().trim().into()))
            .effect(capture.name("effect").map(|s| s.as_str().trim().replace("\r", "").replace("\n", "").into()).unwrap())
            .options(capture.name("option_table").map(|_| vec![load_roll_table(&capture)]).unwrap_or_default())
            .tags(vec![])
            .build()
            .unwrap();
        out.push(cypher)
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
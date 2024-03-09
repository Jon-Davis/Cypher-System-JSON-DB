use std::{fs, sync::OnceLock};

use derive_builder::Builder;
use regex::{Captures, Regex};
use serde::{Serialize, Deserialize};
use unidecode::unidecode;

#[derive(Serialize, Deserialize, Clone)]
pub struct Amount {
    pub tier: usize,
    pub special_abilities: usize,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, Default)]
pub struct RollTable {
    pub name: Option<String>,
    pub description: Option<String>,
    #[builder(setter(each(name = "add_options")))]
    pub table: Vec<RollEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RollEntry {
    pub start: usize,
    pub end: usize,
    pub entry: String,
}

// other patterns can throw this where appropriate to capture a table and then 
// just pass the whole capture to the load_roll_table function
pub const OPTION_TABLE_PATTERN: &'static str = r"(?P<option_table>((?P<named>NAMED\s*)?OPTION TABLE[^\S\r\n]*(?P<option_name>.*)?(\n\s*(?P<option_description>[^\d].*)\s*\n)?(?P<options>[\W\w]*?)))";
const OPTION_ROLL_PATTERN: &'static str = r"(\d+)(-(\d+))? (.*)";

fn get_option_roll_regex() -> &'static Regex {
    static OPTION_ROLL_REGEX: OnceLock<Regex> = OnceLock::new();
    OPTION_ROLL_REGEX.get_or_init(|| Regex::new(OPTION_ROLL_PATTERN).unwrap())
}

pub fn load_intrusion_other_tables() -> (Vec<RollTable>, Vec<RollTable>) {
    let mut intrusions = vec![];
    let mut other = vec![];
    let tables = unidecode(&fs::read_to_string("rand_tables.md").unwrap()).replace("\r", "");
    let regex = Regex::new(&format!(r"(?mi){OPTION_TABLE_PATTERN}\n\n")).unwrap();
    let intrusion_regex = Regex::new(r"(?i)intrusion").unwrap();

    for table_capture in regex.captures_iter(&tables) {
        let table = load_roll_table(&table_capture);
        match &table.name {
            Some(name) if intrusion_regex.is_match(&name) => intrusions.push(table),
            _ => other.push(table)
        };
    }

    (intrusions, other)
}

pub fn load_roll_table(captures: &Captures<'_>) -> RollTable {
    let name = captures.name("option_name").map(|n| n.as_str().to_ascii_uppercase().trim().to_string());
    let description = captures.name("option_description").map(|n| n.as_str().trim().to_string());
    let options = captures.name("options").unwrap().as_str().trim();
    let table = load_option_table(options);
    RollTable {
        name,
        description,
        table
    }
}

pub fn load_option_table(input: &str) -> Vec<RollEntry> {
    let option_regex = get_option_roll_regex();
    let mut out = vec![];
    for line in input.split('\n').map(|s| s.trim()) {
        if option_regex.is_match(line) {
            let captures = option_regex.captures(line).unwrap();
            let start = captures.get(1).map(|s| match s.as_str().trim() {
                "00" => 100,
                s => s.parse().unwrap(),
            }).unwrap();
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
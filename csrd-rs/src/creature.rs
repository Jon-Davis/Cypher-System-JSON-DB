use std::fs;

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;

use crate::tables::*;

#[derive(Builder, Serialize, Deserialize)]
pub struct Creature {
    pub name: String,
    pub kind: String,
    pub level: Option<usize>,
    pub description: String,
    pub motive: Option<String>,
    pub environment: Option<String>,
    pub health: Option<usize>,
    pub damage: Option<String>,
    pub armor: usize,
    pub movement: Option<String>,
    pub modifications: Vec<String>,
    pub combat: Option<String>,
    pub options: Vec<RollTable>,
    pub interactions: Option<String>,
    pub uses: Option<String>,
    pub loot: Option<String>,
    pub intrusions: Option<String>
}

// Named Regex: (?m)(?P<name>[-(),\w\s]*)\s(?P<level>\d*)\s\(\d*\)\s*(?P<description>[\w\W]*?)Motive: (?P<motive>.*)\s*(Environment: (?P<environment>.*)\s*)?Health: (?P<health>\d*)\s*Damage Inflicted: (?P<damage>.*)(\s*Armor: (?P<armor>\d+))?\s*Movement: (?P<movement>.*)\s*(Modifications: (?P<modification>.*)\s*)?(Combat: (?P<combat>[\w\W]*?)Interaction: (?P<interaction>.*)\s*)?(Use: (?P<use>.*)\s*)?(Loot: (?P<loot>.*)\s*)?(GM\s(\(group\)\s)?[iI]ntrusions?:\s(?P<intrusion>.*))?
pub fn load_creatures(file: &str, kind: &str) -> Vec<Creature> {
    let creatures = unidecode(&fs::read_to_string(file).unwrap()).replace("\r", "");
    let mut out = vec![];
    let creature_regex = Regex::new(&format!(r"(?m)(?P<name>[-(),\w\s]*)\s(?P<level>\d*)\s\(\d*\)\s*(?P<description>[[:ascii:]]*?)Motive: (?P<motive>[^\n]*)\s*(?:Environment: (?P<environment>[^\n]*)\s*)?Health: (?P<health>\d*)\s*Damage Inflicted: (?P<damage>[^\n]*)(\s*Armor: (?P<armor>\d+))?\s*Movement: (?P<movement>[^\n]*)\s*(?:Modifications: (?P<modification>[^\n]*)\s*)?(?:Combat: (?P<combat>[[:ascii:]]*?){OPTION_TABLE_PATTERN}?Interaction: (?P<interaction>[^\n]*)\s*)?(?:Use: (?P<use>[^\n]*)\s*)?(?:Loot: (?P<loot>[^\n]*)\s*)?(?:GM\s(\(group\)\s)?[iI]ntrusions?:\s(?P<intrusion>[^\n]*))?")).unwrap();
    assert!(creature_regex.is_match(&creatures));
    for capture in creature_regex.captures_iter(&creatures) {
        let creature = Creature {
            name: capture.name("name").unwrap().as_str().trim().to_ascii_uppercase().into(),
            kind: kind.into(),
            level: capture.name("level").map(|s| s.as_str().parse().unwrap()).filter(|l| *l != 0),
            description: capture.name("description").unwrap().as_str().trim().into(),
            motive: capture.name("motive").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            environment: capture.name("environment").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            health: capture.name("health").map(|s| s.as_str().parse().unwrap_or(0)).filter(|l| *l != 0),
            damage: capture.name("damage").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            armor: capture.name("armor").map(|s| s.as_str().parse().unwrap()).unwrap_or(0),
            movement: capture.name("movement").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            modifications: capture.name("modification").into_iter().flat_map(|s| s.as_str().split(";")).map(|s| s.trim().to_string()).collect(),
            combat: capture.name("combat").map(|s| s.as_str().trim().into()),
            options: capture.name("option_table").map(|_| vec![load_roll_table(&capture)]).unwrap_or_default(),
            interactions: capture.name("interaction").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            uses: capture.name("use").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            loot: capture.name("loot").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            intrusions: capture.name("intrusion").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
        };
        out.push(creature);
    }
    
    out
}

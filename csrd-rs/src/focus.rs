use std::{fs, collections::HashMap};

use regex::Regex;
use serde::{Deserialize, Serialize};
use unidecode::unidecode;
use crate::ability::{Ability, AbilityRef};
use itertools::Itertools;

#[derive(Debug, Serialize, Deserialize)]
pub struct Focus {
    pub name: String,
    pub description: String,
    pub note: Option<String>,
    pub abilities: Vec<AbilityRef>,
    pub connections: Vec<String>,
    pub intrusions: Option<String>,
    pub additional_equipment: Option<String>,
    pub minor_effect: Option<String>,
    pub major_effect: Option<String>
}

pub fn load_foci(abilities_entry: &mut HashMap<String, Ability>) -> Vec<Focus> {
    let foci = unidecode(&fs::read_to_string("Foci.md").unwrap()).replace("\r", "");
    // The "main" regex that separates each focus and breaks it up by section
    let regex = Regex::new(r"(?m)(?P<name>.*?)\n(?P<description>.*)\n(\s*\((?P<note>.*)\)\n)?(?P<connection>Connection:.*\s*(\s*\d.\s*(.*))*\n)?(?P<basic_abilities>(^[^(Tier)].*\s*)*?)?(Type Swap Option: (?P<swap>.*)\n)?(?P<abilities>(^Tier \d:\s*.*\s*)+)(.*?GM Intrusions:(?P<intrusions>.*))?((\n){2,})").unwrap();
    let connections_regex = Regex::new(r"(?m)\s*\d\.\s*(?P<connection>.*)\s*").unwrap();
    let effects_regex = Regex::new(r"(?mi)\s*(?P<name>.*?): (?P<effect>.*)\s*").unwrap();
    // sets the maximum number of abilities that can fit on a single line and 'or'ed together
    const MAX_ABILITIES_PER_LINE : usize = 3; 
    // Compile all abilities into a haystack because abilities like "Captivate or Inspire"
    // might conflict with patterns such as "Tier 3: Golem Stomp or Weaponization"
    let mut abilities : Vec<_> = abilities_entry.keys().collect();
    abilities.sort_by(|a, b| b.cmp(a));
    let abilities_str = abilities.iter().fold(String::new(), |mut acc, a| {
            acc.push_str(&a.trim().replace("?", r"\?"));
            acc.push_str("|");
            acc
        }).trim_end_matches("|").to_owned();
    // duplicates the haystack for each potential ability on the line
    let multi_ability_str = (2..=MAX_ABILITIES_PER_LINE).into_iter().map(|v| format!(r"(\s*or\s*)?(?P<a{v}>{abilities_str})?")).join("");
    // compiles to one giant regex, but it can resolve multiple abilities
    let abilities_regex = Regex::new(&format!(r"(?mi)Tier\s*(?P<tier>\d+)\s*:\s*(?P<a1>{abilities_str}){multi_ability_str}\s*")).unwrap();
    let mut out = vec!();
    assert!(regex.is_match(&foci));
    for capture in regex.captures_iter(&foci) {
        // capture top level fields
        let name = capture.name("name").unwrap().as_str().trim().to_ascii_uppercase();
        let description = capture.name("description").unwrap().as_str().trim().into();
        let intrusions = capture.name("intrusions").map(|s| s.as_str().trim().into());
        let note = capture.name("note").map(|s| s.as_str().trim().into());
        let abilities_str = capture.name("abilities").unwrap().as_str().trim();
        // capture additional_equipment, minor effects and major effects
        let (mut additional_equipment, mut minor_effect, mut major_effect) = (None, None, None);
        if let Some(basic_abilities) = capture.name("basic_abilities").map(|s| s.as_str().trim()) {
            for capture in effects_regex.captures_iter(basic_abilities) {
                match (capture.name("name").map(|s| s.as_str()), capture.name("effect").map(|s| s.as_str().trim())) {
                    (Some("Minor Effect Suggestion"), Some(value)) => minor_effect = Some(value.into()),
                    (Some("Major Effect Suggestion"), Some(value)) => major_effect = Some(value.into()),
                    (Some("Additional Equipment"), Some(value)) => additional_equipment = Some(value.into()),
                    (Some(other), Some(value)) => eprintln!("WARN: Found {other} with value {value} in foci {name}"),
                    _ => (),
                }
            }
        }
        // capture connections
        let mut connections = Vec::new();
        if let Some(connections_str) = capture.name("connection").map(|s| s.as_str().trim()) {
            for capture in connections_regex.captures_iter(connections_str) {
                connections.push(capture.name("connection").unwrap().as_str().trim().into());
            }
        }
        // capture abilities
        let mut abilities = Vec::with_capacity(10);
        // helper fn, add ability to abilities array and a ref to the abilities_entires
        let mut add_ability = |ability : &str, tier : &str, selected| {
            abilities_entry.get_mut(&ability.to_ascii_uppercase())
                .expect(&format!("can't find ref to {:?} in foci {:?}", &ability, &name))
                .references.insert(name.clone());
            abilities.push(AbilityRef {
                name: ability.trim().into(),
                tier: tier.parse().unwrap(),
                preselected: selected,
            })
        };
        // add swap type abilities
        if let Some(swap) = capture.name("swap") {
            add_ability(swap.as_str().trim().into(), "1", false);
        }
        // add normal tiered abilities
        assert!(abilities_regex.is_match(&abilities_str));
        for capture in abilities_regex.captures_iter(abilities_str) {
            let ability_str : Vec<&str> = (1..=MAX_ABILITIES_PER_LINE).into_iter()
                .map(|v| capture.name(&format!("a{v}")))
                .filter_map(|ability| ability.map(|s| s.as_str()))
                .collect();
            for ability in ability_str.iter() {
                add_ability(&ability, capture.name("tier").unwrap().as_str(), ability_str.len() == 1);
            }
        }
        out.push(Focus {
            name,
            description,
            note,
            intrusions,
            connections,
            abilities,
            additional_equipment,
            minor_effect,
            major_effect
        })
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
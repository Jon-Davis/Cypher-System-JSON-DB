use std::{fs, collections::HashMap};

use regex::Regex;
use serde::{Deserialize, Serialize};
use unidecode::unidecode;
use crate::ability::{Ability, AbilityRef};
use itertools::Itertools;

#[derive(Debug, Serialize, Deserialize, Default)]
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

pub fn load_foci(abilities_entry: &HashMap<String, Ability>) -> Vec<Focus> {
    let foci = unidecode(&fs::read_to_string("Foci.md").unwrap()).replace("\r", "");
    // The "main" regex that separates each focus and breaks it up by section
    let regex = Regex::new(r"(?m)(?P<name>[^\n]*?)\n(?P<description>[^\n]*?)\n(?:\s*\((?P<note>[^\n]*)\)\n)?(?P<connection>Connection:.*\s*(?:\s*\d.\s*([^\n]*))*\n)?(?P<basic_abilities>(^[^(Tier)][^\n]*\s*)*?)?(?:Type Swap Option: (?P<swap>[^\n]*)\n)?(?P<abilities>(?:^Tier \d:\s*[^\n]*\s*)+)(?:[^\n]*?GM Intrusions:(?P<intrusions>[^\n]*))?(?:\n{2,})").unwrap();
    let connections_regex = Regex::new(r"(?m)\s*\d\.\s*(?P<connection>[^\n]*)\s*").unwrap();
    let effects_regex = Regex::new(r"(?mi)\s*(?P<name>[^\n]*?): (?P<effect>[^\n]*)\s*").unwrap();
    // sets the maximum number of abilities that can fit on a single line and 'or'ed together
    const MAX_ABILITIES_PER_LINE : usize = 3; 
    // Compile all abilities into a haystack because abilities like "Captivate or Inspire"
    // might conflict with patterns such as "Tier 3: Golem Stomp or Weaponization"
    let mut abilities = abilities_entry.keys().filter(|s| s.contains(" OR "));
    // duplicates the haystack for each potential ability on the line
    let or_abilities = abilities.join("|");
    let mut ars = format!(r"(?mi)Tier\s*(?P<tier>\d+)\s*:\s*(?P<a1>(?:{or_abilities})|[^\n]*?)#end");
    for num in 2..=MAX_ABILITIES_PER_LINE {
        ars = ars.replace(r"#end", &format!(r"(?: or (?P<a{num}>(?:{or_abilities})|[^\n]*?)#end|$)"));
    }
    ars = ars.replace("#end", "$");
    let abilities_regex = Regex::new(&ars).unwrap();
    // compiles to one giant regex, but it can resolve multiple abilities
    let mut out = vec!();
    assert!(regex.is_match(&foci));
    for capture in regex.captures_iter(&foci) {
        // capture top level fields
        let mut focus = Focus::default();
        focus.name = capture.name("name").unwrap().as_str().trim().to_ascii_uppercase();
        focus.description = capture.name("description").unwrap().as_str().trim().into();
        focus.intrusions = capture.name("intrusions").map(|s| s.as_str().trim().into());
        focus.note = capture.name("note").map(|s| s.as_str().trim().into());
        let abilities_str = capture.name("abilities").unwrap().as_str().trim();
        // capture additional_equipment, minor effects and major effects
        if let Some(basic_abilities) = capture.name("basic_abilities").map(|s| s.as_str().trim()) {
            for capture in effects_regex.captures_iter(basic_abilities) {
                match (capture.name("name").map(|s| s.as_str()), capture.name("effect").map(|s| s.as_str().trim())) {
                    (Some("Minor Effect Suggestion"), Some(value)) => focus.minor_effect = Some(value.into()),
                    (Some("Major Effect Suggestion"), Some(value)) => focus.major_effect = Some(value.into()),
                    (Some("Additional Equipment"), Some(value)) => focus.additional_equipment = Some(value.into()),
                    (Some(other), Some(value)) => eprintln!("WARN: Found {other} with value {value} in foci {}", focus.name),
                    _ => (),
                }
            }
        }
        // capture connections
        if let Some(connections_str) = capture.name("connection").map(|s| s.as_str().trim()) {
            for capture in connections_regex.captures_iter(connections_str) {
                focus.connections.push(capture.name("connection").unwrap().as_str().trim().into());
            }
        }
        // helper fn, add ability to abilities array and a ref to the abilities_entires
        let mut add_ability = |ability : &str, tier : &str, selected| {
            let mut map = abilities_entry.get(&ability.to_ascii_uppercase()).expect(&format!("{ability}")).references.lock().unwrap();
            map.insert(focus.name.clone());
            focus.abilities.push(AbilityRef {
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
                add_ability(ability.trim(), capture.name("tier").unwrap().as_str(), ability_str.len() == 1);
            }
        }
        out.push(focus)
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
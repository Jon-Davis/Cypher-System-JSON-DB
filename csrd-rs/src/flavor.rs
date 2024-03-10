use std::{fs, collections::HashMap};

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::ability::{AbilityRef, Ability};

#[derive(PartialEq)]
enum FlavorSM {
    Name,
    Tier(usize),
}

#[derive(Builder, Serialize, Deserialize)]
pub struct Flavor {
    pub name: String,
    pub description: String,
    #[builder(setter(each(name = "add_ability")))]
    pub abilities: Vec<AbilityRef>,
}

pub fn load_flavors(abilities: &HashMap<String, Ability>) -> Vec<Flavor> {
    let types = unidecode(&fs::read_to_string("Flavors.md").unwrap());
    let mut out = vec![];
    let name_regex = Regex::new(r"^(STEALTH FLAVOR|TECHNOLOGY FLAVOR|MAGIC FLAVOR|COMBAT FLAVOR|SKILLS AND KNOWLEDGE FLAVOR)\s*([^\n]*?)$").unwrap();
    let ident_tier = Regex::new(r"^(\d)-TIER [^\n]* ABILITIES$").unwrap();
    let mut phase = FlavorSM::Name;
    let mut current = FlavorBuilder::default();
    for line in types.split('\n').map(|s| s.trim()) {
        if name_regex.is_match(line) {
            let caputres = name_regex.captures(line.trim()).unwrap();
            if phase != FlavorSM::Name {
                out.push(current.build().unwrap());
                current = FlavorBuilder::default();
                phase = FlavorSM::Name;
            }
            current.abilities(vec![]);
            current.name(caputres.get(1).unwrap().as_str().to_ascii_uppercase().into());
            current.description(caputres.get(2).unwrap().as_str().into());
        } else if ident_tier.is_match(line) {
            let cap = ident_tier.captures(line).unwrap();
            phase = FlavorSM::Tier(cap.get(1).unwrap().as_str().parse().unwrap())
        } else if let FlavorSM::Tier(tier) = phase {
            if line.len() > 0 {
                let mut map = abilities.get(&line.to_ascii_uppercase()).unwrap().references.lock().unwrap();
                map.insert(current.name.clone().unwrap());
                current.add_ability(AbilityRef {
                    name: line.into(),
                    preselected: false,
                    tier,
                });
            }
        }
    }
    out.push(current.build().unwrap());
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
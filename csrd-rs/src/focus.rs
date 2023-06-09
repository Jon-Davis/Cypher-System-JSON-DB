use std::{fs, collections::HashMap};

use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::{ability::{Ability, AbilityRef}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Focus {
    pub name: String,
    pub description: String,
    pub abilities: Vec<AbilityRef>,
    pub intrusions: String,
}

pub fn load_foci(abilities_entry: &mut HashMap<String, Ability>) -> Vec<Focus> {
    let foci = unidecode(&fs::read_to_string("Foci.md").unwrap());
    let r36 = Regex::new(r"Tier (\d): (.*) or (.*)").unwrap();
    let r1 = Regex::new(r"Tier (\d): (.*)").unwrap();
    let ri = Regex::new(r"GM Intrusions:(.*)").unwrap();
    let rs = Regex::new(r"Type Swap Option: (.*)").unwrap();
    let mut out = vec!();
    let mut lines = 0;
    let mut abilities : Vec<AbilityRef> = vec![];
    let mut name : Option<String> = None;
    let mut description : Option<String> = None;
    let mut intrustions : Option<String> = None;
    for line in foci.split("\n") {
        if lines == 0 {
            name = Some(line.trim().to_ascii_uppercase().into());
        } else if lines == 1 {
            description = Some(line.trim().into())
        } else if r36.is_match(line) && !line.contains("Captivate or Inspire") {
            let c1 = r36.captures(line.trim()).unwrap();
            let tier : usize = c1.get(1).unwrap().as_str().parse().unwrap();
            let mut ability1 : String = c1.get(2).unwrap().as_str().trim().into();
            if ability1.to_lowercase().starts_with("masterful armor modification") {
                ability1 = "Masterful Armor Modification".into();
            }
            let mut ability2 : String = c1.get(3).unwrap().as_str().trim().into();
            if ability2.to_lowercase().starts_with("masterful armor modification") {
                ability2 = "Masterful Armor Modification".into();
            }
            abilities.push(AbilityRef {name: ability1.clone(), tier, preselected: false});
            abilities.push(AbilityRef {name: ability2.clone(), tier, preselected: false});
            abilities_entry.get_mut(&ability1.to_ascii_uppercase())
                .expect(&format!("couldn't find reference to {:?} in foci {:?}", &ability1.to_ascii_uppercase(), name))
                .references.insert(name.clone().unwrap());
            abilities_entry.get_mut(&ability2.to_ascii_uppercase())
                .expect(&format!("couldn't find reference to {:?} in foci {:?}", &ability2.to_ascii_uppercase(), name))
                .references.insert(name.clone().unwrap());
        } else if r1.is_match(line) {
            let c1 = r1.captures(line.trim()).unwrap();
            let tier : usize = c1.get(1).unwrap().as_str().parse().unwrap();
            let mut ability_name : String = c1.get(2).unwrap().as_str().trim().into();
            if ability_name.starts_with("Greater Skill With Attacks") {
                ability_name = "Greater Skill With Attacks".into();
            }
            abilities.push(AbilityRef {name: ability_name.clone(), tier, preselected: true});
            abilities_entry.get_mut(&ability_name.to_ascii_uppercase())
                .expect(&format!("couldn't find reference to {:?} in foci {:?}", &ability_name.to_ascii_uppercase(), name))
                .references.insert(name.clone().unwrap());
        } else if ri.is_match(line) {
            intrustions = Some(ri.captures(line).unwrap().get(1).unwrap().as_str().trim().into());
        } else if rs.is_match(line) {
            let c1 = rs.captures(line.trim()).unwrap();
            let ability_name : String = c1.get(1).unwrap().as_str().trim().into();
            abilities.push(AbilityRef { name : ability_name.clone(), tier: 1, preselected: false });
            abilities_entry.get_mut(&ability_name.to_ascii_uppercase()).unwrap().references.insert(name.clone().unwrap());
        }
        if line.trim().len() == 0 {
            let focus = Focus {
                name: name.clone().unwrap(),
                description: description.clone().unwrap(),
                intrusions: intrustions.clone().unwrap(),
                abilities: abilities.clone(),
            };
            out.push(focus);
            abilities.clear();
            lines = 0;
        } else {
            lines += 1;
        }
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
use std::{fs, collections::{BTreeSet, HashMap}};

use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicAbility {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityRef {
    pub name: String,
    pub tier: usize,
    pub preselected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub cost: Option<usize>,
    pub pool: Vec<String>,
    pub additional_cost: Option<String>,
    pub cost_rendered: String,
    pub tier: Option<String>,
    pub category: Vec<String>,
    pub description: String,
    pub references: BTreeSet<String>,
}

pub fn load_abilities() -> HashMap<String, Ability> {
    let abilities = unidecode(&fs::read_to_string("Abilities.md").unwrap());
    let abilities_regex = Regex::new(r"(?P<name>^\w[\w\s\d/\-'\?,]*)(?P<rendered>\((?P<cost>\d*)(?P<plus>\+)? (?P<pool>[\w\s]*) points?( \+ (?P<additional>.*))?\))?:(?P<description>.*)").unwrap();
    let mut map : HashMap<String, Ability> = abilities.split('\n')
        .filter(|line| line.trim().len() != 0)
        .filter_map(|line| abilities_regex.captures(line.trim()))
        .map(|captures| 
        (captures.name("name").unwrap().as_str().trim().to_ascii_uppercase(),
        Ability {
            name: captures.name("name").unwrap().as_str().trim().into(),
            cost: captures.name("cost").map(|n| n.as_str().parse().unwrap()),
            pool: captures.name("pool").map(|p| {
                let mut v = vec!();
                if p.as_str().contains("Might") {
                    v.push("Might".into())
                }
                if p.as_str().contains("Speed") {
                    v.push("Speed".into())
                }
                if p.as_str().contains("Intellect") {
                    v.push("Intellect".into())
                }
                v
            }).unwrap_or_default(),
            additional_cost: captures.name("additional").map(|s| s.as_str().trim().into())
                .or(captures.name("plus").map(|s| s.as_str().trim().into())),
            cost_rendered: captures.name("rendered").map(|s| s.as_str().trim().trim_end_matches(")").trim_start_matches("(").into()).unwrap_or_default(),
            description: captures.name("description").unwrap().as_str().trim().into(),
            tier: None,
            category: vec![],
            references: BTreeSet::new(),
        })).collect();
    
    let tiers = fs::read_to_string("AbilityTiers.md").unwrap();
    let mut kind = "";
    let mut tier = "";
    tiers.split("\n").map(|l| l.trim()).filter(|l| l.len() > 1).for_each(|line| {
        if line.starts_with("#") {
            kind = line.trim().strip_prefix("# ").unwrap();
        } else if line.contains("Low Tier:") {
            tier = "Low"
        } else if line.contains("Mid Tier"){
            tier = "Mid"
        } else if line.contains("High Tier"){
            tier = "High"
        } else {
            let value = map.get_mut(&line.to_ascii_uppercase()).expect(&format!("Expected {line}"));
            value.category.push(kind.into());
            value.tier = Some(tier.into())
        }
    });
    map
}
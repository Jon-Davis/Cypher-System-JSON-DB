use std::{fs, collections::HashMap};

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::{ability::{BasicAbility, AbilityRef, Ability}, common_types::*};

#[derive(PartialEq)]
enum TypeStateMachine {
    Name,
    Intrusions,
    StatPool,
    Abilities,
    SpecialAbilities(usize),
}

#[derive(Builder, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
    #[builder(setter(each(name = "add_intrusions")))]
    pub intrusions: Vec<BasicAbility>,
    #[builder(setter(each(name = "add_stat")))]
    #[serde(serialize_with = "ordered_stat_pool")]
    pub stat_pool: HashMap<String, usize>,
    #[builder(setter(each(name = "add_amount")))]
    pub special_abilities_per_tier: Vec<Amount>,
    #[builder(setter(each(name = "add_ability")))]
    pub abilities: Vec<BasicAbility>,
    #[builder(setter(each(name = "add_special")))]
    pub special_abilities: Vec<AbilityRef>,
}

#[derive(Builder, Serialize, Deserialize)]
pub struct StatPool {
    #[serde(rename = "Might")]
    pub might: usize,
    #[serde(rename = "Speed")]
    pub speed: usize,
    #[serde(rename = "Intellect")]
    pub intellect: usize,
}

fn ordered_stat_pool<S>(value: &HashMap<String, usize>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let stat_pool = StatPoolBuilder::default()
        .might(*value.get("Might").unwrap())
        .speed(*value.get("Speed").unwrap())
        .intellect(*value.get("Intellect").unwrap())
        .build().unwrap();
    stat_pool.serialize(serializer)
}

pub fn load_types(abilities: &mut HashMap<String, Ability>) -> Vec<Type> {
    let types = unidecode(&fs::read_to_string("Types.md").unwrap());
    let mut out = vec![];
    let name_regex = Regex::new(r"^(WARRIOR|ADEPT|EXPLORER|SPEAKER)$").unwrap();
    let ident_intrusions = Regex::new(r"^(WARRIOR|ADEPT|EXPLORER|SPEAKER) PLAYER INTRUSIONS$").unwrap();
    let colon_regex = Regex::new(r"^(.*): (.*)$").unwrap();
    let ident_stats = Regex::new(r"^(WARRIOR|ADEPT|EXPLORER|SPEAKER) STAT POOLS$").unwrap();
    let stats_regex = Regex::new(r"^(.*) (\d*)$").unwrap();
    let ident_abilities = Regex::new(r"^1-TIER (WARRIOR|ADEPT|EXPLORER|SPEAKER) ABILITIES").unwrap();
    let ident_tier = Regex::new(r"^(.*)-TIER (WARRIOR|ADEPT|EXPLORER|SPEAKER)$").unwrap();
    let specials_per_tier = Regex::new(r"^Choose (\d) .*$").unwrap();
    let mut phase = TypeStateMachine::Name;
    let mut current = TypeBuilder::default();
    for line in types.split('\n').map(|s| s.trim()) {
        if name_regex.is_match(line) {
            if phase != TypeStateMachine::Name {
                out.push(current.build().unwrap());
                current = TypeBuilder::default();
                phase = TypeStateMachine::Name;
            }
            current.stat_pool(HashMap::new());
            current.abilities(vec![]);
            current.intrusions(vec![]);
            current.special_abilities(vec![]);
            current.special_abilities_per_tier(vec![]);
            current.name(line.to_ascii_uppercase().into());
        } else if ident_intrusions.is_match(line) {
            phase = TypeStateMachine::Intrusions;
        } else if phase == TypeStateMachine::Intrusions && colon_regex.is_match(line) {
            let cap = colon_regex.captures(line).unwrap();
            current.add_intrusions(BasicAbility {
                name: cap.get(1).unwrap().as_str().into(), 
                description: cap.get(2).unwrap().as_str().into()
            });
        } else if ident_stats.is_match(line) {
            phase = TypeStateMachine::StatPool;
        } else if phase == TypeStateMachine::StatPool && stats_regex.is_match(line) {
            let cap = stats_regex.captures(line).unwrap();
            current.add_stat((
                cap.get(1).unwrap().as_str().trim().into(),
                cap.get(2).unwrap().as_str().parse::<usize>().unwrap().into(),
            ));
        } else if ident_abilities.is_match(line) {
            phase = TypeStateMachine::Abilities;
        } else if phase == TypeStateMachine::Abilities && colon_regex.is_match(line) {
            let cap = colon_regex.captures(line).unwrap();
            current.add_ability(BasicAbility {
                name: cap.get(1).unwrap().as_str().into(), 
                description: cap.get(2).unwrap().as_str().into() 
            });
        } else if ident_tier.is_match(line) {
            let cap = ident_tier.captures(line).unwrap();
            phase = TypeStateMachine::SpecialAbilities(cap.get(1).unwrap().as_str().parse().unwrap())
        } else if let TypeStateMachine::SpecialAbilities(tier) = phase {
            if specials_per_tier.is_match(line) {
                let cap = specials_per_tier.captures(line).unwrap();
                current.add_amount(Amount { tier, special_abilities: cap.get(1).unwrap().as_str().parse().unwrap() });
            } else if  line.len() > 0 {
                abilities.get_mut(&line.to_ascii_uppercase()).unwrap().references.insert(current.name.clone().unwrap());
                current.add_special(AbilityRef {
                    name: line.into(),
                    preselected: false,
                    tier,
                });
            }
        }
    }
    out.push(current.build().unwrap());
    out
}
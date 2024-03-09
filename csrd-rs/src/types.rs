use std::{fs, collections::HashMap};

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::{ability::{BasicAbility, AbilityRef, Ability}, tables::*};

#[derive(Builder, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
    #[builder(setter(each(name = "add_intrusions")))]
    pub intrusions: Vec<BasicAbility>,
    pub stat_pool: StatPool,
    pub background: RollTable,
    #[builder(setter(each(name = "add_amount")))]
    pub special_abilities_per_tier: Vec<Amount>,
    #[builder(setter(each(name = "add_ability")))]
    pub abilities: Vec<BasicAbility>,
    #[builder(setter(each(name = "add_special")))]
    pub special_abilities: Vec<AbilityRef>,
}

#[derive(Builder, Serialize, Deserialize, Clone)]
pub struct StatPool {
    #[serde(rename = "Might")]
    pub might: usize,
    #[serde(rename = "Speed")]
    pub speed: usize,
    #[serde(rename = "Intellect")]
    pub intellect: usize,
}

pub fn load_types(abilities: &mut HashMap<String, Ability>) -> Vec<Type> {
    let types = unidecode(&fs::read_to_string("Types.md").unwrap());
    let mut out = vec![];
    let type_regex = Regex::new(r"(?m)(---)?\s*(?P<type>.*)\s*PLAYER INTRUSIONS(?P<intrusions>[\s\w\W]*?)STAT POOLS\s*((?i)might\s*(?P<might>\d+))\s*((?i)speed\s*(?P<speed>\d+))\s*((?i)intellect\s*(?P<intellect>\d+))\s*BACKGROUND(?P<background>[\s\w\W]*?)\s*ABILITIES(?P<abilities>[\w\s\S]*?)(?P<tiers>1-TIER[\s\w\W]*?)---").unwrap();
    let basic_regex = Regex::new(r"(?m)^(?P<name>.*?):\s*(?P<description>.*)$").unwrap();
    let special_regex = Regex::new(r"(?m)(?P<tier>\d)-TIER\s*Choose (?P<amount>\d).*(?P<abilities>[\s\w\W]*?)(^\s*$)").unwrap();
    for capture in type_regex.captures_iter(&types) {
        let name : String = capture.name("type").map(|s| s.as_str().to_uppercase().trim().into()).unwrap();
        let mut new = TypeBuilder::default();
        new.name(name.clone());

        // add player intrusions
        for intrusion in capture.name("intrusions").into_iter().flat_map(|i| basic_regex.captures_iter(i.as_str())) {
            new.add_intrusions(BasicAbility { 
                name: intrusion.name("name").unwrap().as_str().trim().into(), 
                description: intrusion.name("description").unwrap().as_str().trim().into()
            });
        }

        // add starting stats
        let might = capture.name("might").unwrap().as_str().parse::<usize>().unwrap().into();
        let speed = capture.name("speed").unwrap().as_str().parse::<usize>().unwrap().into();
        let intellect = capture.name("intellect").unwrap().as_str().parse::<usize>().unwrap().into();
        new.stat_pool(StatPool{might, speed, intellect});

        // add background
        let entries = load_option_table(capture.name("background").unwrap().as_str());
        new.background(RollTable { name: Some("BACKGROUND".into()), description: None, table: entries });

        // add basic abilities
        for ability in capture.name("abilities").into_iter().flat_map(|i| basic_regex.captures_iter(i.as_str())) {
            new.add_ability(BasicAbility { 
                name: ability.name("name").unwrap().as_str().trim().into(), 
                description: ability.name("description").unwrap().as_str().trim().into()
            });
        }

        // add special abilities
        for ability in capture.name("tiers").into_iter().flat_map(|i| special_regex.captures_iter(i.as_str())) {
            let tier = ability.name("tier").unwrap().as_str().parse().unwrap();
            new.add_amount(Amount { tier, special_abilities: ability.name("amount").unwrap().as_str().parse().unwrap()});

            for ability in ability.name("abilities").unwrap().as_str().trim().split("\n") {
                abilities.get_mut(&ability.trim().to_ascii_uppercase()).unwrap().references.insert(name.clone());
                new.add_special(AbilityRef { name: ability.trim().into(), tier, preselected: false });
            }
        }

        out.push(new.build().unwrap());
    }
    out
}
use std::{fs, collections::{BTreeMap, BTreeSet}};

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;


#[derive(Debug, Serialize, Deserialize, Builder, Clone)]
pub struct EquipmentVariant {
    description: String,
    notes: BTreeSet<String>,
    tags: BTreeSet<String>,
    value: Vec<String>,
    levels: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equipment {
    name: String,
    variants: Vec<EquipmentVariant>,
}

// \n*\s*(.*)\s*\n*\s*(.*)
// $1; Level 0; $2; \n
pub fn load_equipment(file: &str, equipment_codex: &mut BTreeMap<String, Equipment>) {
    let equipment = unidecode(&fs::read_to_string(file).unwrap()).replace("\r", "");
    let equipment_regex = Regex::new(r"(?m)(?:\s*)(?P<tags>(?:[^\n]+))(?P<text>[[:ascii:]]*?)---\s*").unwrap();
    let variant_regex = Regex::new(r"(?m)(?P<name>.*?);\s*((?i)level\s(?P<level>\d*))[^\n]*?;(?P<value>.*?);(?P<description>[^\n]*?)(?P<notes>(?:<<[^\n]*>>)|$)").unwrap();
    let value_seperator = Regex::new(r"(?:\/)|(?: to )").unwrap();
    let note_regex = Regex::new(r"(?m)\s*<<(?P<note>[^\n]*?)>>\s*").unwrap();
    assert!(equipment_regex.is_match(&equipment));
    for capture in equipment_regex.captures_iter(&equipment) {
        let tags : BTreeSet<String> = capture.name("tags").unwrap().as_str().split(";").map(|s| s.trim().into()).collect();
        let text = capture.name("text").unwrap().as_str().trim();

        // loop through each item in the category
        for item in variant_regex.captures_iter(text) {
            let name = item.name("name").unwrap().as_str().trim().to_string();
            let level : usize = item.name("level").unwrap().as_str().parse().unwrap();
            let value = value_seperator.split(item.name("value").unwrap().as_str()).map(|s| s.trim().to_ascii_uppercase()).collect::<Vec<_>>();
            let description = item.name("description").unwrap().as_str().trim().into();
            let notes = note_regex.captures_iter(item.name("notes").map(|s| s.as_str()).unwrap_or_default())
                .filter_map(|s| s.name("note"))
                .map(|s| s.as_str().trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let mut tags = tags.clone();
            if tags.contains("CONTEMPORARY") {
                tags.insert("MODERN".into());
            } else if tags.contains("ADVANCED") || tags.contains("FANTASTIC") {
                tags.insert("SCIENCE FICTION".into());
            }

            // create the new item
            let new_item = EquipmentVariantBuilder::default()
                .description(description)
                .notes(notes)
                .value(value)
                .levels(Some(level).into_iter().filter(|i| *i > 0).collect())
                .tags(tags)
                .build().unwrap();

            // add it as a variant 
            equipment_codex.entry(name.clone())
                .and_modify(|e| {
                    if new_item.tags.contains("CONTEMPORARY") {
                        e.variants.retain(|e| !e.tags.contains("MODERN"));
                    } else if new_item.tags.contains("ADVANCED") || new_item.tags.contains("FANTASTIC") {
                        e.variants.retain(|e| !e.tags.contains("SCIENCE FICTION"));
                    }
                    e.variants.push(new_item.clone())
                })
                .or_insert(Equipment{name, variants: vec![new_item]});
        }
    }
}
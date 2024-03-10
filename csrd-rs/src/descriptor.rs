use std::fs;

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::ability::BasicAbility;

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct Descriptor {
    pub name: String,
    pub description: String,
    #[builder(setter(each(name = "add_ability")))]
    pub characteristics: Vec<BasicAbility>,
    #[builder(setter(each(name = "add_link")))]
    pub links: Vec<String>,
}

pub fn load_descriptors() -> Vec<Descriptor> {
    let decriptors = unidecode(&fs::read_to_string("Descriptors.md").unwrap());
    let descriptor_regex = Regex::new(r"(?m)\s*(?P<name>[^\n]*)(?P<description>[[:ascii:]]*?)You gain the following characteristics:\s*(?P<abilities>[[:ascii:]]*?)(?:Initial Link to the Starting Adventure:[^\n]*\s*(?P<links>[[:ascii:]]*?))?(?:^\s*$)").unwrap();
    let basic_regex = Regex::new(r"(?m)^(?P<name>[^\n]*?):\s*(?P<description>[^\n]*)$").unwrap();
    let link_regex = Regex::new(r"(?m)^\s*\d+\.\s*(?P<link>[^\n]*)").unwrap();
    let mut out = vec!();
    for capture in descriptor_regex.captures_iter(&decriptors) {
        let mut new = DescriptorBuilder::default();
        new.name(capture.name("name").map(|s| s.as_str().to_uppercase().trim().into()).unwrap());
        new.description(capture.name("description").map(|s| s.as_str().trim().into()).unwrap());

        // add basic abilities
        for ability in capture.name("abilities").into_iter().flat_map(|i| basic_regex.captures_iter(i.as_str())) {
            new.add_ability(BasicAbility { 
                name: ability.name("name").unwrap().as_str().trim().into(), 
                description: ability.name("description").unwrap().as_str().trim().into()
            });
        }

        // add links
        new.links(Vec::new());
        for link in capture.name("links").into_iter().flat_map(|i| link_regex.captures_iter(i.as_str())) {
            new.add_link(link.name("link").unwrap().as_str().trim().into());
        }

        out.push(new.build().unwrap());
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
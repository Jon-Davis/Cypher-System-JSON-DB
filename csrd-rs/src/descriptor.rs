use std::fs;

use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use crate::ability::BasicAbility;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Descriptor {
    pub name: String,
    pub description: String,
    pub characteristics: Vec<BasicAbility>,
    pub links: Vec<String>,
}

pub fn load_descriptors() -> Vec<Descriptor> {
    let decriptors = unidecode(&fs::read_to_string("Descriptors.md").unwrap());
    let char_regex = Regex::new(r"([\w\d\s\-\(\),]*): (.*)").unwrap();
    let link_regex = Regex::new(r"\d\. (.*)").unwrap();
    let mut out = vec!();
    let mut lines = 0;
    let mut characteristics : Vec<BasicAbility> = vec![];
    let mut links : Vec<String> = vec![];
    let mut name : Option<String> = None;
    let mut description : Option<String> = None;
    for line in decriptors.split("\n") {
        if lines == 0 {
            name = Some(line.trim().into());
        } else if lines == 1 {
            description = Some(line.trim().into())
        } else if char_regex.is_match(line) {
            let captures = char_regex.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str().trim().to_string();
            let description = captures.get(2).unwrap().as_str().trim().to_string();
            characteristics.push(BasicAbility{name, description});
        } else if link_regex.is_match(line) {
            let captures = link_regex.captures(line).unwrap();
            let link = captures.get(1).unwrap().as_str().trim().to_string();
            links.push(link);
        }
        if line.trim().len() == 0 {
            let focus = Descriptor { 
                name: name.clone().unwrap().to_ascii_uppercase(), 
                description: description.clone().unwrap(), 
                characteristics: characteristics.clone(), 
                links: links.clone(),
            };
            out.push(focus);
            characteristics.clear();
            links.clear();
            lines = 0;
        } else {
            lines += 1;
        }
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
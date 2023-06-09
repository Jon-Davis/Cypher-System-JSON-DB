use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use std::fs;

use crate::common_types::*;

#[derive(Builder, Serialize, Deserialize, Debug)]
pub struct Artifact {
    pub name: String,
    pub level_dice: Option<String>,
    pub level_mod: usize,
    pub form: String,
    pub depletion: String,
    pub effect: String,
    pub options: Vec<RollTable>,
}

// Named Regex: (?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Form:\s*(?P<form>.*)\s*Effect:\s*(?P<effect>[\s\w\W]*?)Depletion:\s*(?P<depletion>.*)\s*
pub fn load_artifacts() -> Vec<Artifact> {
    let cyphers = unidecode(&fs::read_to_string("Artifacts.md").unwrap());
    let cyphers_regex = Regex::new(r"(?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Form:\s*(?P<form>.*)\s*Effect:\s*(?P<effect>[\s\w\W]*?)(OPTION TABLE[^\S\r\n]*(?P<optname>.*)?(?P<options>[[\s\w\W]]*?))?Depletion:\s*(?P<depletion>.*)").unwrap();
    let mut out = vec![];
    for capture in cyphers_regex.captures_iter(&cyphers) {
        let artifact = ArtifactBuilder::default()
            .name(capture.name("name").map(|s| s.as_str().to_uppercase().trim().into()).unwrap())
            .level_dice(capture.name("dice").map(|s| s.as_str().trim().into()))
            .level_mod(capture.name("mod").and_then(|s| s.as_str().parse().ok()).unwrap_or(0))
            .form(capture.name("form").map(|s| s.as_str().trim().into()).unwrap())
            .effect(capture.name("effect").map(|s| s.as_str().trim().replace("\r", "").replace("\n", "").into()).unwrap())
            .options(capture.name("options").map(|s| s.as_str()).map(load_option_table).map(|t| RollTableBuilder::default().table(t).name(capture.name("optname").map(|s| s.as_str().trim().into())).build().unwrap()).into_iter().collect())
            .depletion(capture.name("depletion").map(|s| s.as_str().trim().into()).unwrap())
            .build().unwrap();

        out.push(artifact);
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
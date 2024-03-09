use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;
use std::{fs, collections::BTreeSet};
use crate::tables::*;

#[derive(Builder, Serialize, Deserialize, Debug)]
pub struct Artifact {
    pub name: String,
    pub tags: BTreeSet<String>,
    pub level_dice: Option<String>,
    pub level_mod: usize,
    pub form: String,
    pub depletion: String,
    pub effect: String,
    pub options: Vec<RollTable>,
}

// Named Regex: (?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Form:\s*(?P<form>.*)\s*Effect:\s*(?P<effect>[\s\w\W]*?)Depletion:\s*(?P<depletion>.*)\s*
pub fn load_artifacts() -> Vec<Artifact> {
    let artifacts = unidecode(&fs::read_to_string("Artifacts.md").unwrap());
    let settings_regex = Regex::new(r"(?m)(?P<name>.*)\s*ARTIFACTS\s*(?P<text>[\s\w\W]*?)\s*---").unwrap();
    let artifact_regex = Regex::new(&format!(r"(?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Form:\s*(?P<form>.*)\s*Effect:\s*(?P<effect>[\s\w\W]*?){OPTION_TABLE_PATTERN}?Depletion:\s*(?P<depletion>.*)")).unwrap();
    let mut out = vec![];
    for setting in settings_regex.captures_iter(&artifacts) {
        let tags : BTreeSet<String> = setting.name("name").map(|s| s.as_str().trim().to_string()).into_iter().collect();
        let text = setting.name("text").unwrap().as_str().trim();
        for artifact in artifact_regex.captures_iter(text) {
            let artifact = ArtifactBuilder::default()
                .name(artifact.name("name").map(|s| s.as_str().to_uppercase().trim().into()).unwrap())
                .tags(tags.clone())
                .level_dice(artifact.name("dice").map(|s| s.as_str().trim().into()))
                .level_mod(artifact.name("mod").and_then(|s| s.as_str().parse().ok()).unwrap_or(0))
                .form(artifact.name("form").map(|s| s.as_str().trim().into()).unwrap())
                .effect(artifact.name("effect").map(|s| s.as_str().trim().replace("\r", "").replace("\n", "").into()).unwrap())
                .options(artifact.name("option_table").map(|_| vec![load_roll_table(&artifact)]).unwrap_or_default())
                .depletion(artifact.name("depletion").map(|s| s.as_str().trim().into()).unwrap())
                .build().unwrap();

        out.push(artifact);
        }
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}
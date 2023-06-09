use std::{fs, collections::{HashMap, BTreeSet}};

use derive_builder::Builder;
use regex::Regex;
use serde::{Serialize, Deserialize};
use unidecode::unidecode;

fn main() {
    let mut abilities = load_abilities();
    let foci = load_foci(&mut abilities);
    let descriptors = load_descriptors();
    let types = load_types(&mut abilities);
    let flavors = load_flavors(&mut abilities);
    let mut cyphers = load_cyphers();
    let cypher_tables = load_cypher_tables(&mut cyphers);
    let artifacts = load_artifacts();
    let mut creatures = load_creatures("Creatures.md", "Creature");
    creatures.append(&mut load_creatures("SuperVillains.md", "Super Villain"));
    creatures.append(&mut load_creatures("Npc.md", "NPC"));
    creatures.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    let mut abilities : Vec<Ability> = abilities.into_values().into_iter().collect();
    abilities.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    let db = CsrdDb {
        descriptors, 
        types, 
        foci, 
        abilities, 
        flavors, 
        cyphers, 
        cypher_tables, 
        artifacts, 
        creatures
    };
    let json = serde_json::to_string(&db).unwrap();
    println!("{json}");
    let _new : CsrdDb = serde_json::from_str(&json).unwrap();
}

#[derive(Builder, Serialize, Deserialize, Debug)]
struct Artifact {
    name: String,
    level_dice: Option<String>,
    level_mod: usize,
    form: String,
    depletion: String,
    effect: String,
}

#[derive(Builder, Serialize, Deserialize, Debug)]
struct CypherTable {
    kind: String,
    #[builder(setter(each(name = "add_options")))]
    options: Vec<RollEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RollEntry {
    start: usize,
    end: usize,
    entry: String,
}

#[derive(Builder, Serialize, Deserialize, Debug)]
struct Cypher {
    name: String,
    level_dice: Option<String>,
    level_mod: usize,
    effect: String,
    #[builder(setter(each(name = "add_options")))]
    options: Vec<RollEntry>,
    kinds: Vec<String>,
}

#[derive(PartialEq)]
enum FlavorSM {
    Name,
    Tier(usize),
}

#[derive(Builder, Serialize, Deserialize)]
struct Flavor {
    name: String,
    #[builder(setter(each(name = "add_ability")))]
    abilities: Vec<AbilityRef>,
}

#[derive(Serialize, Deserialize)]
struct CsrdDb {
    types: Vec<Type>,
    flavors: Vec<Flavor>,
    descriptors: Vec<Descriptor>,
    foci: Vec<Focus>,
    abilities: Vec<Ability>,
    cyphers: Vec<Cypher>,
    cypher_tables: Vec<CypherTable>,
    artifacts: Vec<Artifact>,
    creatures: Vec<Creature>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BasicAbility {
    name: String,
    description: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Descriptor {
    name: String,
    description: String,
    characteristics: Vec<BasicAbility>,
    links: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Focus {
    name: String,
    description: String,
    abilities: Vec<AbilityRef>,
    intrusions: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AbilityRef {
    name: String,
    tier: usize,
    preselected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ability {
    name: String,
    cost: Option<usize>,
    pool: Vec<String>,
    additional_cost: Option<String>,
    tier: Option<String>,
    category: Vec<String>,
    description: String,
    references: BTreeSet<String>,
}

#[derive(Builder, Serialize, Deserialize)]
struct Type {
    name: String,
    #[builder(setter(each(name = "add_intrusions")))]
    intrusions: Vec<BasicAbility>,
    #[builder(setter(each(name = "add_stat")))]
    #[serde(serialize_with = "ordered_stat_pool")]
    stat_pool: HashMap<String, usize>,
    #[builder(setter(each(name = "add_amount")))]
    special_abilities_per_tier: Vec<Amount>,
    #[builder(setter(each(name = "add_ability")))]
    abilities: Vec<BasicAbility>,
    #[builder(setter(each(name = "add_special")))]
    special_abilities: Vec<AbilityRef>,
}

#[derive(Builder, Serialize, Deserialize)]
struct StatPool {
    #[serde(rename = "Might")]
    might: usize,
    #[serde(rename = "Speed")]
    speed: usize,
    #[serde(rename = "Intellect")]
    intellect: usize,
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

#[derive(Serialize, Deserialize, Clone)]

struct Amount {
    tier: usize,
    special_abilities: usize,
}

#[derive(PartialEq)]
enum TypeStateMachine {
    Name,
    Intrusions,
    StatPool,
    Abilities,
    SpecialAbilities(usize),
}

#[derive(PartialEq)]
enum CypherSM {
    Name,
    Descriptions,
    Options
}

#[derive(PartialEq)]
enum CypherTableSM {
    Name,
    Option
}

#[derive(Builder, Serialize, Deserialize)]
struct Creature {
    name: String,
    kind: String,
    level: Option<usize>,
    description: String,
    motive: Option<String>,
    environment: Option<String>,
    health: Option<usize>,
    damage: Option<String>,
    armor: usize,
    movement: Option<String>,
    modifications: Vec<String>,
    combat: Option<String>,
    interactions: Option<String>,
    uses: Option<String>,
    loot: Option<String>,
    intrusions: Option<String>
}

// Named Regex: (?m)(?P<name>[-(),\w\s]*)\s(?P<level>\d*)\s\(\d*\)\s*(?P<description>[\w\W]*?)Motive: (?P<motive>.*)\s*(Environment: (?P<environment>.*)\s*)?Health: (?P<health>\d*)\s*Damage Inflicted: (?P<damage>.*)(\s*Armor: (?P<armor>\d+))?\s*Movement: (?P<movement>.*)\s*(Modifications: (?P<modification>.*)\s*)?(Combat: (?P<combat>[\w\W]*?)Interaction: (?P<interaction>.*)\s*)?(Use: (?P<use>.*)\s*)?(Loot: (?P<loot>.*)\s*)?(GM\s(\(group\)\s)?[iI]ntrusions?:\s(?P<intrusion>.*))?
fn load_creatures(file: &str, kind: &str) -> Vec<Creature> {
    let creatures = unidecode(&fs::read_to_string(file).unwrap()).replace("\r", "");
    let mut out = vec![];
    let creature_regex = Regex::new(r"(?m)(?P<name>[-(),\w\s]*)\s(?P<level>\d*)\s\(\d*\)\s*(?P<description>[\w\W]*?)Motive: (?P<motive>.*)\s*(Environment: (?P<environment>.*)\s*)?Health: (?P<health>\d*)\s*Damage Inflicted: (?P<damage>.*)(\s*Armor: (?P<armor>\d+))?\s*Movement: (?P<movement>.*)\s*(Modifications: (?P<modification>.*)\s*)?(Combat: (?P<combat>[\w\W]*?)Interaction: (?P<interaction>.*)\s*)?(Use: (?P<use>.*)\s*)?(Loot: (?P<loot>.*)\s*)?(GM\s(\(group\)\s)?[iI]ntrusions?:\s(?P<intrusion>.*))?").unwrap();
    assert!(creature_regex.is_match(&creatures));
    for capture in creature_regex.captures_iter(&creatures) {
        let creature = Creature {
            name: capture.name("name").unwrap().as_str().trim().to_ascii_uppercase().into(),
            kind: kind.into(),
            level: capture.name("level").map(|s| s.as_str().parse().unwrap()).filter(|l| *l != 0),
            description: capture.name("description").unwrap().as_str().trim().into(),
            motive: capture.name("motive").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            environment: capture.name("environment").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            health: capture.name("health").map(|s| s.as_str().parse().unwrap_or(0)).filter(|l| *l != 0),
            damage: capture.name("damage").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            armor: capture.name("armor").map(|s| s.as_str().parse().unwrap()).unwrap_or(0),
            movement: capture.name("movement").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            modifications: capture.name("modification").into_iter().flat_map(|s| s.as_str().split(";")).map(|s| s.trim().to_string()).collect(),
            combat: capture.name("combat").map(|s| s.as_str().trim().into()),
            interactions: capture.name("interaction").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            uses: capture.name("use").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            loot: capture.name("loot").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
            intrusions: capture.name("intrusion").map(|s| s.as_str().trim().to_string()).filter(|s| !s.is_empty()),
        };
        out.push(creature);
    }
    
    out
}


// Named Regex: (?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Form:\s*(?P<form>.*)\s*Effect:\s*(?P<effect>[\s\w\W]*?)Depletion:\s*(?P<depletion>.*)\s*
fn load_artifacts() -> Vec<Artifact> {
    let cyphers = unidecode(&fs::read_to_string("Artifacts.md").unwrap());
    let cyphers_regex = Regex::new(r"(?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Form:\s*(?P<form>.*)\s*Effect:\s*(?P<effect>[\s\w\W]*?)(OPTION TABLE(?P<options>[[\s\w\W]]*?))?Depletion:\s*(?P<depletion>.*)\s*").unwrap();
    let mut out = vec![];
    for capture in cyphers_regex.captures_iter(&cyphers) {
        let artifact = ArtifactBuilder::default()
            .name(capture.name("name").map(|s| s.as_str().to_uppercase().trim().into()).unwrap())
            .level_dice(capture.name("dice").map(|s| s.as_str().trim().into()))
            .level_mod(capture.name("mod").and_then(|s| s.as_str().parse().ok()).unwrap_or(0))
            .form(capture.name("form").map(|s| s.as_str().trim().into()).unwrap())
            .effect(capture.name("effect").map(|s| s.as_str().trim().replace("\r", "").replace("\n", "").into()).unwrap())
            .depletion(capture.name("depletion").map(|s| s.as_str().trim().into()).unwrap())
            .build().unwrap();

        out.push(artifact);
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}

fn load_cypher_tables(db_cyphers: &mut Vec<Cypher>) -> Vec<CypherTable> {
    let mut out = vec![];
    let cyphers = unidecode(&fs::read_to_string("CypherTables.md").unwrap());
    let option_regex = Regex::new(r"(\d+)(-(\d+))? (.*)").unwrap();
    let mut current = CypherTableBuilder::default();
    let mut phase = CypherTableSM::Name;
    for line in cyphers.split('\n').map(|s| s.trim()) {
        if phase == CypherTableSM::Name {
            current.kind(line.trim().into());
            phase = CypherTableSM::Option;
        } else if option_regex.is_match(line) {
            let captures = option_regex.captures(line).unwrap();
            let start = captures.get(1).unwrap().as_str().parse().unwrap();
            let end = captures.get(3).map(|s| s.as_str().parse().unwrap()).unwrap_or(start);
            let effect : String = captures.get(4).unwrap().as_str().trim().into();
            db_cyphers.iter_mut().filter(|c| c.name == effect.to_ascii_uppercase()).next().expect(&effect).kinds.push(current.kind.clone().unwrap());
            current.add_options(RollEntry { start, end, entry: effect });
        } else if line.is_empty() {
            out.push(current.build().unwrap());
            current = CypherTableBuilder::default();
            phase = CypherTableSM::Name;
        }
    }
    out
}

// Named regex: (?m)(?P<name>.*)\s*Level:\s*(?P<dice>\d*d\d*)?[\s\+]*(?P<mod>\d*)\s*Effect:\s*(?P<effect>.*)\s*(?P<options>OPTION TABLE\s*(?:(?:\d*)-?(?:\d*)?\s(?:.*)\s*)+)?
fn load_cyphers() -> Vec<Cypher> {
    let cyphers = unidecode(&fs::read_to_string("Cyphers.md").unwrap());
    let level_regex = Regex::new(r"Level: (\dd\d)?(( \+ )?(\d*)?)?").unwrap();
    let effect_regex = Regex::new(r"Effect: (.*)").unwrap();
    let option_regex = Regex::new(r"(\d+)(-(\d+))? (.*)").unwrap();
    let mut out = vec![];
    let mut phase = CypherSM::Name;
    let mut current = CypherBuilder::default();
    for line in cyphers.split('\n').map(|s| s.trim()) {
        if phase == CypherSM::Name && line.len() > 0 {
            current.name(line.trim().to_ascii_uppercase().into());
            current.options(vec![]);
            current.kinds(vec![]);
            phase = CypherSM::Descriptions;
        } else if level_regex.is_match(line) && phase != CypherSM::Options {
            let captures = level_regex.captures(line).unwrap();
            current.level_dice(captures.get(1).map(|s| s.as_str().trim().into()));
            current.level_mod(captures.get(4).and_then(|s| s.as_str().trim().parse().ok()).unwrap_or(0));
        } else if effect_regex.is_match(line) {
            let captures = effect_regex.captures(line).unwrap();
            current.effect(captures.get(1).unwrap().as_str().trim().into());
        } else if line.contains("OPTION TABLE") {
            phase = CypherSM::Options;
        } else if phase == CypherSM::Options && option_regex.is_match(line) {
            let captures = option_regex.captures(line).unwrap();
            let start = captures.get(1).unwrap().as_str().trim().parse().unwrap();
            let end = captures.get(3).map(|s| s.as_str().trim().parse().unwrap()).unwrap_or(start);
            let effect = captures.get(4).unwrap().as_str().trim().into();
            current.add_options(RollEntry { start, end, entry: effect });
        } else if line.is_empty() && phase != CypherSM::Name {
            if current.level_dice.is_none() {
                current.level_dice(None);
            }
            if current.level_mod.is_none() {
                current.level_mod(0);
            }
            out.push(current.build().unwrap());
            current = CypherBuilder::default();
            phase = CypherSM::Name;
        }
    }
    out.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
    out
}

fn load_flavors(abilities: &mut HashMap<String, Ability>) -> Vec<Flavor> {
    let types = unidecode(&fs::read_to_string("Flavors.md").unwrap());
    let mut out = vec![];
    let name_regex = Regex::new(r"^(STEALTH FLAVOR|TECHNOLOGY FLAVOR|MAGIC FLAVOR|COMBAT FLAVOR|SKILLS AND KNOWLEDGE FLAVOR)$").unwrap();
    let ident_tier = Regex::new(r"^(\d)-TIER .* ABILITIES$").unwrap();
    let mut phase = FlavorSM::Name;
    let mut current = FlavorBuilder::default();
    for line in types.split('\n').map(|s| s.trim()) {
        if name_regex.is_match(line) {
            if phase != FlavorSM::Name {
                out.push(current.build().unwrap());
                current = FlavorBuilder::default();
                phase = FlavorSM::Name;
            }
            current.abilities(vec![]);
            current.name(line.to_ascii_uppercase().into());
        } else if ident_tier.is_match(line) {
            let cap = ident_tier.captures(line).unwrap();
            phase = FlavorSM::Tier(cap.get(1).unwrap().as_str().parse().unwrap())
        } else if let FlavorSM::Tier(tier) = phase {
            if line.len() > 0 {
                abilities.get_mut(&line.to_ascii_uppercase()).unwrap().references.insert(current.name.clone().unwrap());
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

fn load_types(abilities: &mut HashMap<String, Ability>) -> Vec<Type> {
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

fn load_descriptors() -> Vec<Descriptor> {
    let decriptors = unidecode(&fs::read_to_string("Descriptors.md").unwrap());
    let char_regex = Regex::new(r"([\w\d\s-]*): (.*)").unwrap();
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

fn load_foci(abilities_entry: &mut HashMap<String, Ability>) -> Vec<Focus> {
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

fn load_abilities() -> HashMap<String, Ability> {
    let abilities = unidecode(&fs::read_to_string("Abilities.md").unwrap());
    let abilities_regex = Regex::new(r"(^\w[\w\s\d/\-'\?,]*)(\((\d*)\+? ([\w\s]*) points?( \+ (.*))?\))?:(.*)").unwrap();
    let mut map : HashMap<String, Ability> = abilities.split('\n')
        .filter(|line| line.trim().len() != 0)
        .filter_map(|line| abilities_regex.captures(line.trim()))
        .map(|captures| 
        (captures.get(1).unwrap().as_str().trim().to_ascii_uppercase(),
        Ability {
            name: captures.get(1).unwrap().as_str().trim().into(),
            cost: captures.get(3).map(|n| n.as_str().parse().unwrap()),
            pool: captures.get(4).map(|p| {
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
            additional_cost: captures.get(6).map(|s| s.as_str().trim().into()),
            description: captures.get(7).unwrap().as_str().trim().into(),
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
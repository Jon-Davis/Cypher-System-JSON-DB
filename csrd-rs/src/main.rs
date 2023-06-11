mod ability;
mod artifact;
mod common_types;
mod creature;
mod cyphers;
mod descriptor;
mod equipment;
mod types;
mod flavor;
mod focus;

use std::collections::BTreeMap;

use artifact::{Artifact, load_artifacts};
use cyphers::{Cypher, load_cypher_tables, load_cyphers};
use ability::{Ability, load_abilities};
use descriptor::{load_descriptors, Descriptor};
use equipment::Equipment;
use flavor::{Flavor, load_flavors};
use focus::{Focus, load_foci};
use serde::{Serialize, Deserialize};
use types::{Type, load_types};
use common_types::*;
use creature::{Creature, load_creatures};

use crate::equipment::load_equipment;

#[derive(Serialize, Deserialize)]
struct CsrdDb {
    types: Vec<Type>,
    flavors: Vec<Flavor>,
    descriptors: Vec<Descriptor>,
    foci: Vec<Focus>,
    abilities: Vec<Ability>,
    cyphers: Vec<Cypher>,
    cypher_tables: Vec<RollTable>,
    artifacts: Vec<Artifact>,
    creatures: Vec<Creature>,
    equipment: Vec<Equipment>,
}

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
    let mut equipment = BTreeMap::new();
    load_equipment("equipment.md", &mut equipment);
    let equipment = equipment.into_values().collect();
    let db = CsrdDb {
        descriptors, 
        types, 
        foci, 
        abilities, 
        flavors, 
        cyphers, 
        cypher_tables, 
        artifacts, 
        creatures,
        equipment
    };
    let json = serde_json::to_string(&db).unwrap();
    println!("{json}");
    let _new : CsrdDb = serde_json::from_str(&json).unwrap();
}

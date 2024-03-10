mod ability;
mod artifact;
mod tables;
mod creature;
mod cyphers;
mod descriptor;
mod equipment;
mod types;
mod flavor;
mod focus;

use std::{collections::BTreeMap, thread};

use artifact::{Artifact, load_artifacts};
use cyphers::{Cypher, load_cypher_tables, load_cyphers};
use ability::{Ability, load_abilities};
use descriptor::{load_descriptors, Descriptor};
use equipment::Equipment;
use flavor::{Flavor, load_flavors};
use focus::{load_foci, Focus};
use serde::{Serialize, Deserialize};
use types::{Type, load_types};
use tables::*;
use creature::{Creature, load_creatures};
use time::{Date, Instant, OffsetDateTime};
use crossbeam::channel::bounded;
use unidecode::unidecode;
use crate::equipment::load_equipment;

#[derive(Serialize, Deserialize, Default)]
struct CsrdDb {
    version: Option<Date>,
    types: Vec<Type>,
    flavors: Vec<Flavor>,
    descriptors: Vec<Descriptor>,
    foci: Vec<Focus>,
    abilities: Vec<Ability>,
    cyphers: Vec<Cypher>,
    cypher_tables: Vec<RollTable>,
    intrusion_tables: Vec<RollTable>,
    other_tables: Vec<RollTable>,
    artifacts: Vec<Artifact>,
    creatures: Vec<Creature>,
    equipment: Vec<Equipment>,
}

fn main() {
    let start = Instant::now();
    let mut db = CsrdDb {
        version: Some(OffsetDateTime::now_utc().date()),
        ..Default::default()
    };

    let (creatures_tx, creatures_rx) = bounded(3);
    thread::scope(|s| {
        s.spawn(|| {
            let abilities_map = load_abilities();
            thread::scope(|s| {
                s.spawn(|| db.foci = load_foci(&abilities_map));
                s.spawn(|| db.types = load_types(&abilities_map));
                s.spawn(|| db.flavors = load_flavors(&abilities_map));
            });
            db.abilities = abilities_map.into_values().into_iter().collect();
            db.abilities.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        });
        s.spawn(|| db.descriptors = load_descriptors());
        s.spawn(|| {
            db.cyphers = load_cyphers();
            db.cypher_tables = load_cypher_tables(&mut db.cyphers);
        });
        s.spawn(|| db.artifacts = load_artifacts());
        s.spawn(|| {
            let mut equipment_map = BTreeMap::new();
            load_equipment("equipment.md", &mut equipment_map);
            db.equipment = equipment_map.into_values().collect();
        });
        s.spawn(|| (db.intrusion_tables, db.other_tables) = load_intrusion_other_tables());
        s.spawn(|| creatures_tx.send(load_creatures("Creatures.md", "Creature")).unwrap());
        s.spawn(|| creatures_tx.send(load_creatures("SuperVillains.md", "Super Villain")).unwrap());
        s.spawn(|| creatures_tx.send(load_creatures("Npc.md", "NPC")).unwrap());
        s.spawn(|| {
            db.creatures = creatures_rx.iter().take(3).flatten().collect();
            db.creatures.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
        });
    });
    let json = unidecode(&serde_json::to_string(&db).unwrap().replace("\\r", "").replace("\r", ""));
    println!("{json}");
    let end = Instant::now();
    eprintln!("Finished in (roughly) {:?}", end - start);
    let _new : CsrdDb = serde_json::from_str(&json).unwrap();
}

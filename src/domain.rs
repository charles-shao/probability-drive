use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use weighted::RarityCategory;

pub mod rng_table;
pub mod weighted;

#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub weight: usize,
}

lazy_static! {
    // TODO: stick this in a csv file
    static ref ENTITY_PULL_DATA: HashMap<RarityCategory, Vec<Entity>> = {
        let mut common_entities = Vec::<Entity>::with_capacity(7);
        common_entities.push(Entity {
            name: String::from("C Operator 02"),
            weight: 100,
        });
        common_entities.push(Entity {
            name: String::from("C Operator 08"),
            weight: 100,
        });
        common_entities.push(Entity {
            name: String::from("C Operator 15"),
            weight: 100,
        });
        common_entities.push(Entity {
            name: String::from("C Operator 21"),
            weight: 100,
        });
        common_entities.push(Entity {
            name: String::from("C Equipment 04"),
            weight: 100,
        });
        common_entities.push(Entity {
            name: String::from("C Equipment 09"),
            weight: 100,
        });
        common_entities.push(Entity {
            name: String::from("C Equipment 29"),
            weight: 100,
        });

        let mut uncommon_entities = Vec::<Entity>::with_capacity(2);
        uncommon_entities.push(Entity {
            name: String::from("UC Equipment 58"),
            weight: 100,
        });
        uncommon_entities.push(Entity {
            name: String::from("UC Equipment 63"),
            weight: 100,
        });

        let mut rare_entities = Vec::<Entity>::with_capacity(4);
        rare_entities.push(Entity {
            name: String::from("R Operator 17"),
            weight: 100,
        });
        rare_entities.push(Entity {
            name: String::from("R Operator 23"),
            weight: 100,
        });
        rare_entities.push(Entity {
            name: String::from("R Operator 46"),
            weight: 100,
        });
        rare_entities.push(Entity {
            name: String::from("R Operator 87"),
            weight: 100,
        });

        let mut super_rare_entities = Vec::<Entity>::with_capacity(1);
        super_rare_entities.push(Entity {
            name: String::from("SR Operator 04"),
            weight: 100,
        });

        let mut exceptional_entities = Vec::<Entity>::with_capacity(1);
        exceptional_entities.push(Entity {
            name: String::from("EX Equipment 01"),
            weight: 100,
        });

        let mut unique_entities = Vec::<Entity>::with_capacity(1);
        unique_entities.push(Entity {
            name: String::from("UQ Equipment 01"),
            weight: 100,
        });

        let mut m = HashMap::new();
        m.insert(RarityCategory::Common, common_entities);
        m.insert(RarityCategory::Uncommon, uncommon_entities);
        m.insert(RarityCategory::Rare, rare_entities);
        m.insert(RarityCategory::SuperRare, super_rare_entities);
        m.insert(RarityCategory::Exceptional, exceptional_entities);
        m.insert(RarityCategory::Unique, unique_entities);

        m
    };
}

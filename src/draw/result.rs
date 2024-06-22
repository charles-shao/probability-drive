use chrono::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Entity;

pub const WEIGHTS: [i32; 6] = [600, 350, 100, 40, 9, 1];

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum RarityCategory {
    Common,
    Uncommon,
    Rare,
    SuperRare,
    Exceptional,
    Unique,
}

impl TryFrom<usize> for RarityCategory {
    type Error = ();

    fn try_from(n: usize) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(RarityCategory::Common),
            1 => Ok(RarityCategory::Uncommon),
            2 => Ok(RarityCategory::Rare),
            3 => Ok(RarityCategory::SuperRare),
            4 => Ok(RarityCategory::Exceptional),
            5 => Ok(RarityCategory::Unique),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeightMap {
    rarity_category: RarityCategory,
    weight: i32,
}

pub struct Weighted {
    count: usize,
    results: Vec<PullResult>,
}

#[derive(Serialize, Deserialize)]
struct PullResult {
    // credit_eqiv: i32,
    // duplicate: bool,
    entity: Entity,
    rarity_category: RarityCategory,
}

impl Weighted {
    pub fn new(count: usize) -> Weighted {
        assert!(count > 0);

        let mut results: Vec<PullResult> = Vec::with_capacity(count);

        let dist: WeightedIndex<i32> = WeightedIndex::new(&WEIGHTS).unwrap();
        let mut rng: ThreadRng = thread_rng();

        for _ in 0..count {
            let rarity_category: RarityCategory =
                match RarityCategory::try_from(dist.sample(&mut rng)) {
                    Ok(category) => category,
                    Err(_) => {
                        println!("out of bounds index {}", count);
                        RarityCategory::Common
                    }
                };

            let empty_entities: Vec<Entity> = Vec::<Entity>::new();
            let entities: &Vec<Entity> = match super::ENTITY_PULL_DATA.get(&rarity_category) {
                Some(entities) => entities,
                None => {
                    println!("no entities for rarity {:?}", &rarity_category);
                    &empty_entities
                }
            };

            let weights: Vec<i32> = entities.iter().map(|entity| entity.weight as i32).collect();
            let dist: WeightedIndex<i32> = WeightedIndex::new(&weights).unwrap();

            results.push(PullResult {
                entity: entities[dist.sample(&mut rng)].clone(),
                rarity_category: rarity_category,
            });
        }

        Weighted { count, results }
    }

    pub fn to_json(&self) -> String {
        let results: &Vec<PullResult> = &self.results;
        let uuid: Uuid = Uuid::now_v7();
        let json: serde_json::Value = serde_json::json!({
            "count": self.count,
            "generated_at": generate_datetime(),
            "results": results,
            "uuid": uuid.to_string(),
        });

        match ::serde_json::to_string_pretty(&json) {
            Ok(value) => value,
            Err(_) => json.to_string(),
        }
    }
}

fn generate_datetime() -> String {
    let generated_at: DateTime<Utc> = Utc::now();
    generated_at.format("%Y-%m-%dT%H:%M:%S").to_string()
}

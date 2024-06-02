use chrono::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const WEIGHTS: [i32; 6] = [1, 1, 1, 1, 1, 1];

#[derive(Debug, Serialize, Deserialize)]
pub struct WeightMap {
    kind: &'static str,
    weight: i32,
}

pub fn weight_map() -> serde_json::Value {
    let mut weight_map: Vec<WeightMap> = Vec::with_capacity(6);

    for i in 0..6 {
        weight_map.push(WeightMap {
            kind: super::RARITY_CATEGORIES[i],
            weight: WEIGHTS[i],
        })
    }

    serde_json::json!({
        "type": "weighted",
        "mapping": weight_map,
    })
}

#[derive(Debug)]
pub struct Selection {
    count: usize,
    generated_at: DateTime<Utc>,
    results: Vec<PullResult>,
    uuid: Uuid,
}

#[derive(Debug)]
struct PullResult {
    name: &'static str,
}

impl Selection {
    pub fn new(count: usize) -> Selection {
        assert!(count > 0);

        let uuid: Uuid = Uuid::now_v7();
        let generated_at: DateTime<Utc> = Utc::now();
        let mut results: Vec<PullResult> = Vec::with_capacity(count);
        let dist: WeightedIndex<i32> = WeightedIndex::new(&WEIGHTS).unwrap();
        let mut rng: ThreadRng = thread_rng();

        for _ in 0..count {
            results.push(PullResult {
                name: super::RARITY_CATEGORIES[dist.sample(&mut rng)],
            });
        }

        Selection {
            count,
            generated_at,
            results,
            uuid,
        }
    }

    pub fn to_json(&self) -> String {
        let results: &Vec<PullResult> = &self.results;
        let mut names: Vec<&str> = Vec::with_capacity(self.count);

        for result in results {
            names.push(result.name);
        }

        let formatted_date: String = self.generated_at.format("%Y-%m-%dT%H:%M:%S").to_string();

        let json: serde_json::Value = serde_json::json!({
            "count": self.count,
            "generated_at": formatted_date,
            "results": names,
            "uuid": self.uuid.to_string(),
        });

        match ::serde_json::to_string_pretty(&json) {
            Ok(value) => value,
            Err(_) => json.to_string(),
        }
    }
}

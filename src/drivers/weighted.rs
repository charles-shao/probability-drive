use chrono::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const CHOICES: [&str; 6] = [
    "Common",
    "Uncommon",
    "Rare",
    "Super Rare",
    "Exceptional",
    "Unique",
];

pub const WEIGHTS: [i32; 6] = [600, 350, 100, 40, 9, 1];

#[derive(Debug, Serialize, Deserialize)]
pub struct ChanceMap {
    choice: &'static str,
    weight: i32,
}

pub fn chance_map_to_json() -> String {
    let mut chance_map: Vec<ChanceMap> = Vec::with_capacity(6);

    for i in 0..6 {
        chance_map.push(
            ChanceMap {
                choice: CHOICES[i],
                weight: WEIGHTS[i]
            }
        )
    }

    let json: serde_json::Value = serde_json::json!({
        "type": "weighted",
        "mapping": chance_map,
    });

    json.to_string()
}

pub struct Weighted {
    count: usize,
    generated_at: DateTime<Utc>,
    results: Vec<PullResult>,
    uuid: Uuid,
}

struct PullResult {
    name: &'static str,
}

impl Weighted {
    pub fn new(count: usize) -> Weighted {
        assert!(count > 0);

        let uuid: Uuid = Uuid::now_v7();
        let generated_at: DateTime<Utc> = Utc::now();
        let mut results: Vec<PullResult> = Vec::with_capacity(count);
        let dist: WeightedIndex<i32> = WeightedIndex::new(&WEIGHTS).unwrap();
        let mut rng: ThreadRng = thread_rng();

        for _ in 0..count {
            results.push(PullResult {
                name: CHOICES[dist.sample(&mut rng)],
            });
        }

        Weighted {
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

        let formatted_date: String = self.generated_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let json: serde_json::Value = serde_json::json!({
            "count": self.count,
            "generated_at": formatted_date,
            "results": names,
            "uuid": self.uuid.to_string(),
        });

        json.to_string()
    }
}

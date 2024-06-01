use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub struct Weighted {
    count: usize,
    results: Vec<PullResult>,
}

struct PullResult {
    name: &'static str,
}

impl Weighted {
    pub fn new(count: usize) -> Weighted {
        assert!(count > 0);

        let mut results: Vec<PullResult> = Vec::with_capacity(count);

        let choices: [&str; 6] = [
            "Common",
            "Uncommon",
            "Rare",
            "Super Rare",
            "Exceptional",
            "Unique",
        ];
        let weights: [i32; 6] = [600, 350, 100, 40, 9, 1];
        let dist: WeightedIndex<i32> = WeightedIndex::new(&weights).unwrap();
        let mut rng: ThreadRng = thread_rng();

        for _ in 0..count {
            results.push(PullResult {
                name: choices[dist.sample(&mut rng)],
            });
        }

        Weighted { count, results }
    }

    pub fn to_json(&self) -> String {
        let results: &Vec<PullResult> = &self.results;
        let mut names: Vec<&str> = Vec::with_capacity(self.count);

        for result in results {
            names.push(result.name);
            println!("{}", result.name);
        }

        let json: serde_json::Value = serde_json::json!({
            "results": names,
            "count": self.count
        });

        json.to_string()
    }
}

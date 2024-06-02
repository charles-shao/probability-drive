use rand::prelude::*;

pub fn generate() -> Vec<u8> {
    let mut table: Vec<u8> = (0..=255).collect();
    let mut thread_rng: ThreadRng = rand::thread_rng();

    table.shuffle(&mut thread_rng);

    table
}

extern crate rand;
use crate::rand::Rng;

use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    for i in 2..10 {
        let min: i64 = 0;
        let max: i64 = i64::pow(10, i);

        let mut rng = StdRng::seed_from_u64(42); // the answer to life

        let mut nums = vec![];

        for _ in min..max {
            let n: i64 = rng.gen_range(min..max);
            nums.push(n);
        }

        let mut buffer = File::create(format!("benchmark-{i}.txt", i = i)).unwrap();
        for num in nums {
            buffer
                .write((num.to_string() + "\n").as_bytes())
                .expect("Failed to write to buffer");
        }
    }
}

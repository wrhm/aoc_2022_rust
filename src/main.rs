mod day_01;
mod day_02;
mod util;

use std::time::Instant;

// Run `cargo test` from ./ (src/) or ../ (aoc_2022/)
// Run `cargo run` from ./ (src/)
fn main() {
    let now = Instant::now();
    day_01::day_01("../data/01.txt");
    day_02::day_02("../data/02.txt");
    let elapsed = now.elapsed();
    println!("Ran all (implemented) solutions in {:?}", elapsed);
}

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod util;

use std::time::Instant;

// Run `cargo test` from ./ (src/) or ../ (aoc_2022/)
// Run `cargo run` from ./ (src/)
fn main() {
    let now = Instant::now();
    day_01::day_01("../data/01.txt");
    day_02::day_02("../data/02.txt");
    day_03::day_03("../data/03.txt");
    day_04::day_04("../data/04.txt");
    day_05::day_05("../data/05.txt");
    day_06::day_06("../data/06.txt");
    let elapsed = now.elapsed();
    println!("Ran all (implemented) solutions in {:?}", elapsed);
}

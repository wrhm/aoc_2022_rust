mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod util;

use std::time::Instant;

// Lint coverage and tests are checked at commit time via
// ../.git/hooks/pre-commit. Hook source:
// https://deaddabe.fr/blog/2021/09/29/git-pre-commit-hook-for-rust-projects/

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
    day_07::day_07("../data/07.txt");
    day_08::day_08("../data/08.txt");
    day_09::day_09("../data/09.txt");
    let elapsed = now.elapsed();
    println!("Ran all (implemented) solutions in {:?}", elapsed);
}

mod day_01;
mod day_02;
mod util;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day_01::day_01();
    day_02::day_02();
    let elapsed = now.elapsed();
    println!("Ran all (implemented) solutions in {:?}", elapsed);
}

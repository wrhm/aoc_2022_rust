use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::time::Instant;

pub(crate) fn get_file_contents(filename: &str) -> String {
    let path = Path::new(filename);

    let mut file = match File::open(path) {
        Err(why) => panic!("Failed to open file at {}: {}", filename, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("{}", why)
    }
    s
}

// Runs `impl_f` on the contents of the file at `filename`, and prints timing
// results.
pub(crate) fn day_n_i32_i32(filename: &str, two_digit_day: &str, impl_f: fn(&str) -> (i32, i32)) {
    let now = Instant::now();
    let file_contents = get_file_contents(filename);
    let (ans1, ans2) = impl_f(&file_contents);
    let elapsed = now.elapsed();
    println!("Day {}: {}, {}. {:?}", two_digit_day, ans1, ans2, elapsed);
}

pub(crate) fn day_n_str_str(
    filename: &str,
    two_digit_day: &str,
    impl_f: fn(&str) -> (String, String),
) {
    let now = Instant::now();
    let file_contents = get_file_contents(filename);
    let (ans1, ans2) = impl_f(&file_contents);
    let elapsed = now.elapsed();
    println!("Day {}: {}, {}. {:?}", two_digit_day, ans1, ans2, elapsed);
}

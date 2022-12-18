use crate::util;

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

type Point = (i32, i32);

fn day_15_both_parts(file_contents: &str, row: i32, col_min: i32, col_max: i32) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    // let mut sensors: HashSet<Point> = HashSet::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut closest_beacon: HashMap<Point, Point> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\-?\d+").unwrap();
        }

        // (sx,sy,bx,by)
        let matches: Vec<i32> = RE
            .find_iter(line)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .collect();
        let sx = matches[0];
        let sy = matches[1];
        let bx = matches[2];
        let by = matches[3];

        // sensors.insert((sx, sy));
        beacons.insert((bx, by));
        closest_beacon.insert((sx, sy), (bx, by));
    }
    let mut ans1 = 0;
    let mut left_most = 999999999;
    let mut right_most = -999999999;
    for col in col_min..col_max {
        // 4961647
        let current: Point = (col, row);
        if beacons.contains(&current) {
            // println!("({},{}) is a beacon", col, row);

            left_most = std::cmp::min(left_most, col);
            right_most = std::cmp::max(right_most, col);
            continue;
        }
        let mut is_seen: bool = false;
        for (s, b) in &closest_beacon {
            let sensor_reach = (s.0 - b.0).abs() + (s.1 - b.1).abs();
            let current_d = (s.0 - current.0).abs() + (s.1 - current.1).abs();
            if current_d <= sensor_reach {
                // println!(
                //     "({},{}) is within reach of station at ({},{})",
                //     col,
                //     row, s.0, s.1
                // );
                is_seen = true;
                left_most = std::cmp::min(left_most, col);
                right_most = std::cmp::max(right_most, col);
                break;
                // ans1 += 1;
                // println!("count is now {}", ans1);
            }
        }
        if is_seen {
            ans1 += 1;
        }
    }
    println!("left_most: {}, right_most: {}", left_most, right_most);
    let ans2 = 0;
    (ans1, ans2)
}

pub(crate) fn day_15(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    // for col in 0..6000000 { // 4262661
    let million = 1000000;
    // for col in (-1*million)..(8*million) { // 4961647

    // let low_bound = -2 * million;
    // let high_bound = 10 * million;
    let low_bound = -million;
    let high_bound = 5 * million;
    let (ans1, ans2) = day_15_both_parts(&file_contents, 2000000, low_bound, high_bound);
    let elapsed = now.elapsed();
    println!("Day 15: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_15_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/15.txt");
        let (ans1, _) = day_15_both_parts(&file_contents, 10, -50, 50);
        assert_eq!(ans1, 26);
        // assert_eq!(ans2, 56000011);
    }
}

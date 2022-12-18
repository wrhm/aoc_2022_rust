use crate::util;

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

type Point = (i32, i32);

fn day_15_both_parts(file_contents: &str, row: i32) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

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

        beacons.insert((bx, by));
        closest_beacon.insert((sx, sy), (bx, by));
    }

    type Interval = (i32, i32);
    // inclusive ranges of columns where there cannot be a beacon on `row`.
    let mut forbidden_ranges: HashSet<Interval> = HashSet::new();
    let mut beacons_on_row: HashSet<i32> = HashSet::new();
    for (s, b) in &closest_beacon {
        if b.1 == row {
            beacons_on_row.insert(b.0);
        }
        let sensor_reach = (s.0 - b.0).abs() + (s.1 - b.1).abs();
        let (s_col, s_row) = s;
        let vert = (s_row - row).abs();
        if vert > sensor_reach {
            // sensor can't reach row
            continue;
        }

        let range_min = s_col - (sensor_reach - vert);
        let range_max = s_col + (sensor_reach - vert);
        forbidden_ranges.insert((range_min, range_max));
    }
    let mut frv: Vec<Interval> = forbidden_ranges.into_iter().collect();
    frv.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<Interval> = vec![];
    for (a, b) in frv {
        if merged.is_empty() {
            merged.push((a, b));
            continue;
        }
        let &(c, d) = merged.last().unwrap();
        if (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b) {
            merged.pop();
            merged.push((std::cmp::min(a, c), std::cmp::max(b, d)));
        } else {
            merged.push((a, b));
        }
    }
    let mut merged_tot = 0 - (beacons_on_row.len() as i32);
    for (a, b) in merged {
        merged_tot += b - a + 1;
    }
    let ans1 = merged_tot;
    let ans2 = 0;
    (ans1, ans2)
}

pub(crate) fn day_15(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_15_both_parts(&file_contents, 2000000);
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
        let (ans1, _) = day_15_both_parts(&file_contents, 10);
        assert_eq!(ans1, 26);
        // assert_eq!(ans2, 56000011);
    }
}

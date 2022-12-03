use crate::util;

use std::time::Instant;

pub(crate) fn day_01() {
    let now = Instant::now();
    let s = util::get_file_contents("data/01.txt");
    let lines: Vec<&str> = s.split('\n').collect();

    let mut vecs: Vec<Vec<i32>> = vec![];
    let mut vec: Vec<i32> = vec![];
    for line in &lines {
        let val_or = line.parse::<i32>();
        if val_or.is_ok() {
            vec.push(val_or.unwrap());
        } else {
            vecs.push(vec);
            vec = vec![];
        }
    }
    if !vec.is_empty() {
        vecs.push(vec);
    }
    let mut sums: Vec<i32> = vec![];
    for x in &vecs {
        sums.push(x.iter().sum());
    }
    // sort decreasing
    sums.sort_by(|a, b| b.cmp(a));
    let ans1 = sums.iter().max().unwrap();
    let ans2 = sums[0] + sums[1] + sums[2];
    let elapsed = now.elapsed();
    println!("Day 01: {}, {}. {:?}", ans1, ans2, elapsed);
}

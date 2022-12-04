use crate::util;

use std::time::Instant;

fn day_01_impl(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    // parse file into groups of food items held by each elf
    let mut vecs: Vec<Vec<i32>> = vec![];
    let mut vec: Vec<i32> = vec![];
    for line in &lines {
        let val_or = line.parse::<i32>();
        if let Ok(..) = val_or {
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
    // sort decreasing to find three largest
    sums.sort_by(|a, b| b.cmp(a));
    let ans1 = sums[0];
    let ans2 = sums[0] + sums[1] + sums[2];
    (ans1, ans2)
}

pub(crate) fn day_01(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_01_impl(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 01: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_01_impl;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/01.txt");
        let (ans1, ans2) = day_01_impl(&file_contents);
        assert_eq!(ans1, 24000);
        assert_eq!(ans2, 45000);
    }
}

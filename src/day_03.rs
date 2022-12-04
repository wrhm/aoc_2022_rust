use crate::util;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

fn priority(c: char) -> i32 {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let va = 'a' as i32;
    let vz = 'z' as i32;
    let v_cap_a = 'A' as i32;
    let u = c as i32;
    if va <= u && u <= vz {
        u - va + 1
    } else {
        u - v_cap_a + 27
    }
}

fn day_03_impl(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    let mut sum = 0;
    for line in &lines {
        let n = line.len();
        let nh = n / 2;
        let mut left_counts: HashMap<char, i32> = HashMap::new();
        let mut right_counts: HashMap<char, i32> = HashMap::new();
        for (i, c) in line.chars().enumerate() {
            if i < nh {
                *left_counts.entry(c).or_insert(1) += 1;
            } else {
                *right_counts.entry(c).or_insert(1) += 1;
            }
        }

        let mut c = 'A';
        while c <= 'z' {
            let vl = if left_counts.contains_key(&c) {
                *left_counts.get(&c).unwrap()
            } else {
                0
            };
            let vr = if right_counts.contains_key(&c) {
                *right_counts.get(&c).unwrap()
            } else {
                0
            };
            if (vl > 0) && (vr > 0) {
                sum += priority(c);
            }
            if c == 'Z' {
                c = 'a'
            } else {
                c = ((c as u8) + 1) as char;
            }
        }
    }

    let mut sum2 = 0;
    let mut lines_iter = lines.into_iter();
    loop {
        let mut g: Vec<&str> = vec![];
        for _ in 0..3 {
            let x = lines_iter.next();
            if let Some(..) = x {
                g.push(x.unwrap());
            }
        }
        if g.len() < 3 {
            break;
        }
        let mut has_letter: HashMap<char, i32> = HashMap::new();
        for ln in g {
            let mut hs: HashSet<char> = HashSet::new();
            for c in ln.chars() {
                hs.insert(c);
            }
            for c in hs {
                *has_letter.entry(c).or_insert(0) += 1;
            }
        }
        for (c, v) in has_letter {
            if v == 3 {
                sum2 += priority(c);
            }
        }
    }

    let ans1 = sum;
    let ans2 = sum2;
    (ans1, ans2)
}

pub(crate) fn day_03(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_03_impl(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 03: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_03_impl;
    use super::priority;
    use crate::util;

    #[test]
    fn priority_test() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/03.txt");
        let (ans1, ans2) = day_03_impl(&file_contents);
        assert_eq!(ans1, 157);
        assert_eq!(ans2, 70);
    }
}

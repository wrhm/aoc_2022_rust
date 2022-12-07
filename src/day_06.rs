use crate::util;

use std::collections::{HashSet, VecDeque};
use std::time::Instant;

fn find_marker(s: &str, n: i32) -> i32 {
    let mut q: VecDeque<char> = VecDeque::from([]);
    let mut ret = -1;
    for (i, c) in s.chars().enumerate() {
        if q.len() >= n as usize {
            q.pop_front();
        }
        q.push_back(c);
        if q.len() < n as usize {
            continue;
        }
        let mut hs: HashSet<char> = HashSet::new();
        for e in &q {
            hs.insert(*e);
        }
        if hs.len() == n as usize {
            ret = (i + 1) as i32;
            break;
        }
    }
    ret
}

fn day_06_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let ans1 = find_marker(lines.first().unwrap(), 4);
    let ans2 = find_marker(lines.first().unwrap(), 14);
    (ans1, ans2)
}

pub(crate) fn day_06(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_06_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 06: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_06_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents_a = util::get_file_contents("test_data/06a.txt");
        let (ans1_a, ans2_a) = day_06_both_parts(&file_contents_a);
        assert_eq!(ans1_a, 5);
        assert_eq!(ans2_a, 23);

        let file_contents_b = util::get_file_contents("test_data/06b.txt");
        let (ans1_b, ans2_b) = day_06_both_parts(&file_contents_b);
        assert_eq!(ans1_b, 6);
        assert_eq!(ans2_b, 23);

        let file_contents_c = util::get_file_contents("test_data/06c.txt");
        let (ans1_c, ans2_c) = day_06_both_parts(&file_contents_c);
        assert_eq!(ans1_c, 10);
        assert_eq!(ans2_c, 29);

        let file_contents_d = util::get_file_contents("test_data/06d.txt");
        let (ans1_d, ans2_d) = day_06_both_parts(&file_contents_d);
        assert_eq!(ans1_d, 11);
        assert_eq!(ans2_d, 26);
    }
}

use crate::util;

use std::time::Instant;

fn day_04_impl(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let mut count = 0;
    let mut count2 = 0;
    for line in lines {
        let ranges: Vec<&str> = line.split(',').collect();
        if ranges.len() != 2 {
            continue;
        }
        let left_nums: Vec<&str> = ranges[0].split('-').collect();
        let right_nums: Vec<&str> = ranges[1].split('-').collect();
        let a = left_nums[0].parse::<i32>().unwrap();
        let b = left_nums[1].parse::<i32>().unwrap();
        let c = right_nums[0].parse::<i32>().unwrap();
        let d = right_nums[1].parse::<i32>().unwrap();

        // [a,b] is within [c,d] or [c,d] is within [a,b]
        if (a >= c && b <= d) || (c >= a && d <= b) {
            count += 1;
        }
        // c is within [a,b] or a is within [c,d]
        if (a <= c && b >= c) || (c <= a && d >= a) {
            count2 += 1;
        }
    }
    let ans1 = count;
    let ans2 = count2;
    (ans1, ans2)
}

// TODO: abstract out into util by passing a fn arg, with overloads for each return type (two ints, two strs).
pub(crate) fn day_04(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_04_impl(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 04: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_04_impl;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/04.txt");
        let (ans1, ans2) = day_04_impl(&file_contents);
        assert_eq!(ans1, 2);
        assert_eq!(ans2, 4);
    }
}

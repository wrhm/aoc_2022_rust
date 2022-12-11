use crate::util;

use std::time::Instant;

fn day_10_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let mut x = 1;
    let mut cycle = 1;
    let mut strength_sum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
        let nval = if parts.len() == 2 {
            parts[1].parse::<i32>().unwrap()
        } else {
            0
        };
        if parts[0] == "noop" {
            if cycle % 40 == 20 {
                let strength = cycle * x;
                println!("noop: {}*{}={}", cycle, x, strength);
                strength_sum += strength;
            }
            cycle += 1;
        } else {
            if cycle % 40 == 20 {
                let strength = cycle * x;
                println!("addx: {}*{}={}", cycle, x, strength);
                strength_sum += strength;
            }
            cycle += 1;
            if cycle % 40 == 20 {
                let strength = cycle * x;
                println!("addx after tick: {}*{}={}", cycle, x, strength);
                strength_sum += strength;
            }
            cycle += 1;
            x += nval;
        }
    }
    let ans1 = strength_sum;
    let ans2 = 0;
    (ans1, ans2)
}

pub(crate) fn day_10(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_10_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 10: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_10_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/10.txt");
        let (ans1, _) = day_10_both_parts(&file_contents);
        assert_eq!(ans1, 13140);
        // assert_eq!(ans2, -1);
    }
}

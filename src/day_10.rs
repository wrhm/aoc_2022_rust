use crate::util;

use std::time::Instant;

fn update_strength_sum(cycle: i32, x: i32, strength_sum: &mut i32) {
    if cycle % 40 == 20 {
        *strength_sum += cycle * x;
    }
}

fn update_output(cycle: i32, x: i32, out_chars: &mut Vec<char>) {
    if ((cycle - 1) % 40 - x).abs() <= 1 {
        out_chars.push('#');
    } else {
        out_chars.push('.');
    }
    if out_chars.len() == 40 {
        let s: String = out_chars.clone().into_iter().collect();
        println!("{}", s);
        out_chars.clear();
    }
}

fn day_10_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let mut x = 1;
    let mut cycle = 1;
    let mut strength_sum = 0;
    let mut out_chars: Vec<char> = vec![];
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
            update_strength_sum(cycle, x, &mut strength_sum);
            update_output(cycle, x, &mut out_chars);
            cycle += 1;
        } else {
            update_strength_sum(cycle, x, &mut strength_sum);
            update_output(cycle, x, &mut out_chars);
            cycle += 1;
            update_strength_sum(cycle, x, &mut strength_sum);
            update_output(cycle, x, &mut out_chars);
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
    let (ans1, _) = day_10_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!(
        "Day 10: {}, <see output printed above>. {:?}",
        ans1, elapsed
    );
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
        // The answer to part 2 is printed to stdout and needs to be visually
        // inspected.
        // assert_eq!(ans2, -1);
    }
}

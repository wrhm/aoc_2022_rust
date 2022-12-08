use crate::util;

use std::collections::HashMap;
use std::time::Instant;

fn interpret_line(
    line: &str,
    path_segments: &mut Vec<String>,
    filepaths: &mut HashMap<String, i32>,
) {
    let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
    let p0 = parts[0];
    let p1 = parts[1];
    let p0c0 = p0.chars().next().unwrap();
    if p0c0 == '$' {
        // ignore `$ ls`
        if p1 != "cd" {
            return;
        }
        let p2 = parts[2];
        if p2 == "/" {
            // $ cd /
            // path_segments = vec![];
            path_segments.clear();
        } else if p2 == ".." {
            // $ cd ..
            path_segments.pop();
        } else {
            // $ cd x
            path_segments.push(p2.to_string());
        }
    } else if p0c0 != 'd' {
        // <size> <filename>. ignore `dir [...]`
        let mut dir_segs = path_segments.clone();
        dir_segs.push(p1.to_string());
        let sz = p0.parse::<i32>().unwrap();
        filepaths.insert(dir_segs.join("/"), sz);
    }
}

fn calculate_directory_sizes(filepaths: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut totals: HashMap<String, i32> = HashMap::new();
    for (k, v) in filepaths {
        let mut parts: Vec<&str> = k.split('/').collect();
        parts.pop(); // exclude filename portion of path
        loop {
            let partial_prefix = parts.join("/");
            if !parts.is_empty() {
                *totals.entry(partial_prefix).or_insert(0) += v;
                parts.pop();
            }
            // handle the root directory
            if parts.is_empty() {
                *totals.entry("".to_string()).or_insert(0) += v;
                break;
            }
        }
    }
    totals
}

const MAX_ALLOWED_DIR_SIZE: i32 = 100000;
const TOTAL_DISK: i32 = 70000000;
const SPACE_FOR_UPDATE: i32 = 30000000;

fn smallest_overrun(totals: &HashMap<String, i32>, lower_bound: i32) -> i32 {
    // avoid sorting by doing a linear seek and track the smallest overrun.
    let mut smallest_overrun = TOTAL_DISK;
    let mut ret = 0;
    for s in totals.values() {
        if *s < lower_bound {
            continue;
        }
        let overrun = s - lower_bound;
        if overrun < smallest_overrun {
            smallest_overrun = overrun;
            ret = *s;
        }
    }
    ret
}

fn day_07_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let mut path_segments: Vec<String> = vec![];
    // stores size of each file (using full filepath)
    let mut filepaths: HashMap<String, i32> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        interpret_line(line, &mut path_segments, &mut filepaths);
    }

    let totals = calculate_directory_sizes(&filepaths);
    let ans1: i32 = totals
        .values()
        .filter(|&v| *v <= MAX_ALLOWED_DIR_SIZE)
        .sum();

    let total_used = *totals.get("").unwrap(); // size of root ("/")
    let unused = TOTAL_DISK - total_used;
    let min_to_delete = SPACE_FOR_UPDATE - unused;
    let ans2 = smallest_overrun(&totals, min_to_delete);

    (ans1, ans2)
}

pub(crate) fn day_07(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_07_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 07: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_07_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/07.txt");
        let (ans1, ans2) = day_07_both_parts(&file_contents);
        assert_eq!(ans1, 95437);
        assert_eq!(ans2, 24933642);
    }
}

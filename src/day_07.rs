use crate::util;

use std::collections::HashMap;
use std::time::Instant;

// find all of the directories with a total size of at most 100000, then
// calculate the sum of their total sizes

// Idea: build a hashmap of filepath to size. Then walk this map, for each
// path, increase in a second map all subpaths by the filesize. Filter the
// keys of the second map by the max-size constraint.
fn day_07_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let mut path_segments: Vec<String> = vec![];
    let mut filepaths: HashMap<String, i32> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
        let p0 = parts[0];
        let p1 = parts[1];
        let p0c0 = p0.chars().next().unwrap();
        if p0c0 == '$' {
            if p1 == "cd" {
                let p2 = parts[2];
                if p2 == "/" {
                    // $ cd /
                    path_segments = vec![];
                } else if p2 == ".." {
                    // $ cd ..
                    path_segments.pop();
                } else {
                    // $ cd x
                    path_segments.push(p2.to_string());
                }
            } else {
                // $ ls
                continue;
            }
        } else if p0c0 == 'd' {
            // dir <directory>
            let mut dir_segs = path_segments.clone();
            dir_segs.push(p1.to_string());
        } else {
            // <size> <filename>
            let mut dir_segs = path_segments.clone();
            dir_segs.push(p1.to_string());
            let sz = p0.parse::<i32>().unwrap();
            filepaths.insert(dir_segs.join("/"), sz);
        }
    }

    let mut totals: HashMap<String, i32> = HashMap::new();
    for (k, v) in &filepaths {
        let mut parts: Vec<&str> = k.split('/').collect();
        parts.pop(); // exclude filename portion of path
        loop {
            let partial_prefix = parts.join("/");
            if !parts.is_empty() {
                *totals.entry(partial_prefix).or_insert(0) += v;
                parts.pop();
            }
            if parts.is_empty() {
                *totals.entry("".to_string()).or_insert(0) += v;
                break;
            }
        }
    }

    let total_disk = 70000000;
    let space_for_update = 30000000;
    let max_allowed_dir_size = 100000;
    let mut ans1 = 0;
    for v in totals.values() {
        if *v <= max_allowed_dir_size {
            ans1 += v;
        }
    }

    let total_used = *totals.get("").unwrap(); // size of root ("/")
    let unused = total_disk - total_used;
    let min_to_delete = space_for_update - unused;

    let mut ans2 = 0;
    let mut sizes: Vec<i32> = totals.into_values().collect();
    sizes.sort();
    for s in sizes {
        if s >= min_to_delete {
            ans2 = s;
            break;
        }
    }

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

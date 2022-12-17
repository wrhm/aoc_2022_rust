use crate::util;

use std::{collections::HashSet, time::Instant};

// returns true iff grain came to rest
fn drop_grain(drop_loc: (i32, i32), max_row: i32, hs: &mut HashSet<(i32, i32)>) -> bool {
    let (mut c, mut r) = drop_loc;
    loop {
        if r > max_row {
            return false;
        }
        if hs.contains(&(c, r)) {
            return true;
        }
        if !hs.contains(&(c, r + 1)) {
            r += 1;
            continue;
        }
        if hs.contains(&(c, r + 1)) && hs.contains(&(c - 1, r + 1)) && hs.contains(&(c + 1, r + 1))
        {
            // came_to_rest = true;
            hs.insert((c, r));
            break;
        } else if hs.contains(&(c, r + 1)) && hs.contains(&(c - 1, r + 1)) {
            c += 1;
            r += 1;
        } else if hs.contains(&(c, r + 1)) {
            c -= 1;
            r += 1;
        }
    }
    true
}

fn day_14_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    // (col,row)
    let mut hs: HashSet<(i32, i32)> = HashSet::new();
    let mut occupied_rows: HashSet<i32> = HashSet::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<Vec<i32>> = line
            .split(" -> ")
            .map(|x| x.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
            .collect();
        for i in 0..parts.len() - 1 {
            let (c1, r1) = (parts[i][0], parts[i][1]);
            let (c2, r2) = (parts[i + 1][0], parts[i + 1][1]);
            if c1 == c2 {
                for j in std::cmp::min(r1, r2)..std::cmp::max(r1, r2) + 1 {
                    hs.insert((c1, j));
                    occupied_rows.insert(j);
                }
            } else {
                for j in std::cmp::min(c1, c2)..std::cmp::max(c1, c2) + 1 {
                    hs.insert((j, r1));
                    occupied_rows.insert(r1);
                }
            }
        }
    }
    let mut max_occ_row = 0;
    for r in occupied_rows {
        max_occ_row = std::cmp::max(max_occ_row, r);
    }
    let mut hs_part1 = hs.clone();
    let mut ans1 = 0;
    while drop_grain((500, 0), max_occ_row, &mut hs_part1) {
        ans1 += 1;
    }

    let mut hs_part2 = hs.clone();
    for c in 300..700 {
        hs_part2.insert((c, max_occ_row + 2));
    }

    let mut ans2 = 0;
    while !hs_part2.contains(&(500, 0)) && drop_grain((500, 0), max_occ_row + 2, &mut hs_part2) {
        ans2 += 1;
    }

    (ans1, ans2)
}

pub(crate) fn day_14(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_14_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 14: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_14_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/14.txt");
        let (ans1, ans2) = day_14_both_parts(&file_contents);
        assert_eq!(ans1, 24);
        assert_eq!(ans2, 93);
    }
}

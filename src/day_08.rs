use crate::util;

use std::time::Instant;

fn day_08_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let w = lines.first().unwrap().len();
    let mut h = lines.len();
    // println!("(w,h): ({},{})", w, h);
    let mut trees: Vec<Vec<char>> = vec![];

    // `v`isible `f`rom a side
    let mut vf: Vec<Vec<bool>> = vec![];
    for line in lines {
        if line.is_empty() {
            h -= 1;
            continue;
        }
        let chrs: Vec<char> = line.chars().collect();
        trees.push(chrs);
        let falses: Vec<bool> = (0..w).into_iter().map(|_| false).collect();
        vf.push(falses);
    }

    // from top to bottom
    // println!("=== TOP ===");
    for c in 0..w {
        let mut running_max: i32 = -1;
        for r in 0..h {
            // println!("(r,c): ({},{})", r, c);
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                // println!("{} at row {}, col {} is visible from TOP", dig, r, c);
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // from bottom to top
    // println!("=== BOTTOM ===");
    for c in 0..w {
        let mut running_max: i32 = -1;
        for r in (0..h).rev() {
            // println!("(r,c): ({},{})", r, c);
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                // println!("{} at row {}, col {} is visible from BOTTOM", dig, r, c);
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // from left to right
    // println!("=== LEFT ===");
    for r in 0..h {
        let mut running_max: i32 = -1;
        for c in 0..w {
            // println!("(r,c): ({},{})", r, c);
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                // println!("{} at row {}, col {} is visible from LEFT", dig, r, c);
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // from right to left
    // println!("=== RIGHT ===");
    for r in 0..h {
        let mut running_max: i32 = -1;
        for c in (0..w).rev() {
            // println!("(r,c): ({},{})", r, c);
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                // println!("{} at row {}, col {} is visible from RIGHT", dig, r, c);
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // let ans1 = vf
    //     .into_iter()
    //     .map(|r| r.into_iter().map(|el| if el { 1 } else { 0 }).sum())
    //     .sum();
    // let ans1: i32 = vf.into_iter().map(|row| row.into_iter().filter(|x| *x));
    let mut ans1 = 0;
    for row in vf {
        for el in row {
            if el {
                ans1 += 1;
            }
        }
    }
    let ans2 = 0;
    (ans1, ans2)
}

pub(crate) fn day_08(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_08_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 08: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_08_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/08.txt");
        let (ans1, _) = day_08_both_parts(&file_contents);
        assert_eq!(ans1, 21);
        // assert_eq!(ans2, 8);
    }
}

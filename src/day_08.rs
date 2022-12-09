use crate::util;

use std::time::Instant;

fn day_08_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let w = lines.first().unwrap().len();
    let mut h = lines.len();
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

    // === part 1 ===

    // from top to bottom
    for c in 0..w {
        let mut running_max: i32 = -1;
        for r in 0..h {
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // from bottom to top
    for c in 0..w {
        let mut running_max: i32 = -1;
        for r in (0..h).rev() {
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // from left to right
    for r in 0..h {
        let mut running_max: i32 = -1;
        for c in 0..w {
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    // from right to left
    for r in 0..h {
        let mut running_max: i32 = -1;
        for c in (0..w).rev() {
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                vf[r][c] = true;
                running_max = dig;
            }
        }
    }

    let ans1 = vf.iter().flat_map(|v| v.iter()).map(|b| *b as i32).sum();

    // === part 2 ===
    let mut ans2 = 0;

    for r in 0..h {
        for c in 0..w {
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            let mut view_dist_prod = 1;

            // go up
            let view_dist_up;
            let mut r2 = r as i32 - 1;
            loop {
                if r2 < 0 {
                    view_dist_up = (r as i32 - r2 as i32).abs() - 1;
                    break;
                }
                let other_dig = trees[r2 as usize][c].to_digit(10).unwrap() as i32;
                if other_dig >= dig {
                    view_dist_up = (r as i32 - r2 as i32).abs();
                    break;
                }
                r2 -= 1;
            }
            view_dist_prod *= view_dist_up;

            // go down
            let view_dist_down;
            let mut r2 = r as i32 + 1;
            loop {
                if r2 > h as i32 - 1 {
                    view_dist_down = (r as i32 - r2 as i32).abs() - 1;
                    break;
                }
                let other_dig = trees[r2 as usize][c].to_digit(10).unwrap() as i32;
                if other_dig >= dig {
                    view_dist_down = (r as i32 - r2 as i32).abs();
                    break;
                }
                r2 += 1;
            }
            view_dist_prod *= view_dist_down;

            // go left
            let view_dist_left;
            let mut c2 = c as i32 - 1;
            loop {
                if c2 < 0 {
                    view_dist_left = (c as i32 - c2 as i32).abs() - 1;
                    break;
                }
                let other_dig = trees[r][c2 as usize].to_digit(10).unwrap() as i32;
                if other_dig >= dig {
                    view_dist_left = (c as i32 - c2 as i32).abs();
                    break;
                }
                c2 -= 1;
            }
            view_dist_prod *= view_dist_left;

            // go right
            let view_dist_right;
            let mut c2 = c as i32 + 1;
            loop {
                if c2 > w as i32 - 1 {
                    view_dist_right = (c as i32 - c2 as i32).abs() - 1;
                    break;
                }
                let other_dig = trees[r][c2 as usize].to_digit(10).unwrap() as i32;
                if other_dig >= dig {
                    view_dist_right = (c as i32 - c2 as i32).abs();
                    break;
                }
                c2 += 1;
            }
            view_dist_prod *= view_dist_right;

            if view_dist_prod > ans2 {
                ans2 = view_dist_prod;
            }
        }
    }

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
        let (ans1, ans2) = day_08_both_parts(&file_contents);
        assert_eq!(ans1, 21);
        assert_eq!(ans2, 8);
    }
}

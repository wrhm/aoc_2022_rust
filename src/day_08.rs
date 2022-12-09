use crate::util;

// use std::ops::Range;
use std::time::Instant;

// fn traverse_trees(
//     trees: &mut Vec<Vec<char>>,
//     vf: &mut Vec<Vec<bool>>,
//     // row_iter: &mut Range<usize>,
//     row_iter: Range<usize>,
//     col_iter: Range<usize>,
// ) {
//     for c in col_iter {
//         let mut running_max: i32 = -1;
//         // for r in &mut *row_iter {
//         for r in row_iter.clone() {
//             println!("(r,c): ({},{})", r, c);
//             let dig = trees[r][c].to_digit(10).unwrap() as i32;
//             if dig > running_max {
//                 println!("{} at row {}, col {} is visible from TOP", dig, r, c);
//                 vf[r][c] = true;
//                 running_max = dig;
//             }
//         }
//     }
// }

fn day_08_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let w = lines.first().unwrap().len();
    let mut h = lines.len();
    // println!("(w,h): ({},{})", w, h);
    let mut trees: Vec<Vec<char>> = vec![];
    // let mut vis_dists: Vec<Vec<(i32,i32,i32,i32)>> = vec![];
    let mut vis_dist_prods: Vec<Vec<i32>> = vec![];

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
        let ones: Vec<i32> = (0..w).into_iter().map(|_| 1).collect();
        vis_dist_prods.push(ones);
    }

    // from top to bottom
    // println!("=== TOP ===");
    for c in 0..w {
        let mut running_max: i32 = -1;
        // let mut ind_of_max: i32 = -1;
        for r in 0..h {
            // println!("(r,c): ({},{})", r, c);
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            if dig > running_max {
                // println!("{} at row {}, col {} is visible from TOP", dig, r, c);
                vf[r][c] = true;
                running_max = dig;
                // ind_of_max = r as i32;
            }
        }
    }
    // traverse_trees(&mut trees, &mut vf, 0..h, 0..w);

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
    // traverse_trees(&mut trees, &mut vf, 0..h, 0..w);

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

    // part 2
    let mut ans2 = 0;

    for r in 0..h {
        // let mut running_max: i32 = -1;
        for c in 0..w {
            let dig = trees[r][c].to_digit(10).unwrap() as i32;
            let mut view_dist_prod = 1;

            // go up
            // println!("=== UP ===");
            let view_dist_up;
            let mut r2 = r as i32 - 1;
            // stop at taller tree or edge
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
            // println!(
            //     "view_dist UP at (r={},c={}):{} is {}",
            //     r, c, dig, view_dist_up,
            // );

            // go down
            // println!("=== DOWN ===");
            let view_dist_down;
            let mut r2 = r as i32 + 1;
            // stop at taller tree or edge
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
            // println!(
            //     "view_dist DOWN at (r={},c={}):{} is {}",
            //     r, c, dig, view_dist_down,
            // );

            // go left
            // println!("=== LEFT ===");
            let view_dist_left;
            let mut c2 = c as i32 - 1;
            // stop at taller tree or edge
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
            // println!(
            //     "view_dist LEFT at (r={},c={}):{} is {}",
            //     r, c, dig, view_dist_left,
            // );

            // go right
            // println!("=== RIGHT ===");
            let view_dist_right;
            let mut c2 = c as i32 + 1;
            // stop at taller tree or edge
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
            // println!(
            //     "view_dist RIGHT at (r={},c={}):{} is {}",
            //     r, c, dig, view_dist_right,
            // );

            // println!(
            //     "view_dist_prod at (r={},c={}) is {}*{}*{}*{}={}",
            //     r, c, view_dist_up, view_dist_down, view_dist_left, view_dist_right, view_dist_prod
            // );

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

use crate::util;

use std::{collections::HashSet, time::Instant};

fn update_rope(mut dhx: i32, mut dhy: i32, pts: &mut Vec<(i32, i32)>, do_print: bool) {
    if do_print {
        // println!("dhx={},dhy={}", dhx, dhy);
    }
    for i in 0..(pts.len() - 1 + 1) {
        let (mut hx, mut hy) = pts[i];
        if i == pts.len() - 1 {
            pts[i] = (pts[i].0 + dhx, pts[i].1 + dhy);
            continue;
        }
        let (tx, ty) = pts[i + 1];
        let mut new_tx = tx;
        let mut new_ty = ty;

        // let new_dhx; // = tx;
        // let new_dhy; // = ty;
        if do_print {
            // println!("index {}", i);
        };
        // println!(
        //     "H moving from ({},{}) to ({},{})",
        //     hx,
        //     hy,
        //     hx + dhx,
        //     hy + dhy
        // );
        hx += dhx;
        hy += dhy;
        // let mut new_tx = hx;
        // let mut new_ty = hy;
        // horizontal
        if dhy == 0 && ty == hy && ((hx - tx) as i32).abs() >= 2 {
            if do_print {
                // println!("Moving T horizontally")
            };
            new_tx = tx + (hx - tx) / 2;
            // new_dhx = (hx-tx/2);
        }
        // vertical
        else if dhx == 0 && tx == hx && ((hy - ty) as i32).abs() >= 2 {
            if do_print {
                // println!("Moving T vertically")
            };
            new_ty = ty + (hy - ty) / 2;
            // new_dhy = (hy-ty/2);
        }
        // diag dist of 2
        else if ((hx - tx) as i32).abs() >= 2 && ((hy - ty) as i32).abs() >= 2 {
            new_tx = (hx + tx) / 2;
            new_ty = (hy + ty) / 2;
        }
        // move T diagonally
        else if ((hx - tx) as i32).abs() >= 2 {
            if do_print {
                // println!("Moving T diagonally since xs had gap")
            };
            new_tx = (hx + tx) / 2;
            new_ty = hy;
            // new_dhx =
        } else if ((hy - ty) as i32).abs() >= 2 {
            if do_print {
                // println!("Moving T diagonally since ys had gap")
            };
            new_ty = (hy + ty) / 2;
            new_tx = hx;
        } else if ((hx - tx) as i32).abs() <= 1 && ((hy - ty) as i32).abs() <= 1 {
            // println!("Points are already close enough");
        } else {
            println!(
                "Motion unaccounted for at H=pts[{}]: new H: ({},{}), current T: ({},{})",
                i, hx, hy, tx, ty
            );
        }
        let new_dhx = new_tx - tx;
        let new_dhy = new_ty - ty;
        // pts[i] = (hx + dhx, hy + dhy);

        pts[i] = (hx, hy);
        // pts[i + 1] = (new_tx, new_ty);

        dhx = new_dhx;
        dhy = new_dhy;

        // dhx = new_tx - tx;
        // dhy = new_ty - ty;
        // if do_print {
        //     println!("rope #{}: {:?}", pts.len(), pts);
        // }
    }
    if do_print {
        // println!("rope #{}: {:?}", pts.len(), pts);
    }
}

fn day_09_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let mut commands: Vec<(char, i32)> = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
        commands.push((
            parts[0].chars().next().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        ));
    }

    // let mut hx: i32; // = 0;
    // let mut hy = 0;
    // let mut tx = 0;
    // let mut ty = 0;

    let mut dhx: i32;
    let mut dhy: i32;

    let mut rope: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    let mut rope_9: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0), // #10
    ];

    let mut t_locs: HashSet<(i32, i32)> = HashSet::new();
    t_locs.insert((0, 0));
    let mut rope_tail_locs: HashSet<(i32, i32)> = HashSet::new();
    rope_tail_locs.insert((0, 0));
    let mut rope_9_tail_locs: HashSet<(i32, i32)> = HashSet::new();
    rope_9_tail_locs.insert((0, 0));
    let mut rope_9_tail_vec = vec![(0, 0)];
    for (dir, dist) in commands {
        println!("dir: {}", dir);
        if dir == 'R' {
            dhx = 1;
            dhy = 0;
        } else if dir == 'L' {
            dhx = -1;
            dhy = 0;
        } else if dir == 'U' {
            dhx = 0;
            dhy = 1;
        } else {
            dhx = 0;
            dhy = -1;
        }
        for _ in 0..dist {
            // at least one of dhx and dhy will be zero

            // println!(
            //     "T=({},{}). H moving from ({},{}) to ({},{})",
            //     tx,
            //     ty,
            //     hx,
            //     hy,
            //     hx + dhx,
            //     hy + dhy
            // );
            // hx += dhx;
            // hy += dhy;

            // // horizontal
            // if dhy == 0 && ty == hy && ((hx - tx) as i32).abs() >= 2 {
            //     println!("Moving T horizontally");
            //     tx += (hx - tx) / 2;
            // }
            // // vertical
            // else if dhx == 0 && tx == hx && ((hy - ty) as i32).abs() >= 2 {
            //     println!("Moving T vertically");
            //     ty += (hy - ty) / 2;
            // }
            // // move T diagonally
            // else if ((hx - tx) as i32).abs() >= 2 {
            //     println!("Moving T diagonally since xs had gap");
            //     tx = (hx + tx) / 2;
            //     ty = hy;
            // } else if ((hy - ty) as i32).abs() >= 2 {
            //     println!("Moving T diagonally since ys had gap");
            //     ty = (hy + ty) / 2;
            //     tx = hx;
            // }

            update_rope(dhx, dhy, &mut rope, false);
            rope_tail_locs.insert(*rope.last().unwrap());

            update_rope(dhx, dhy, &mut rope_9, true);
            let end_loc = *rope_9.last().unwrap();
            rope_9_tail_locs.insert(end_loc);
            if end_loc != *rope_9_tail_vec.last().unwrap() {
                rope_9_tail_vec.push(end_loc);
                // println!("tail_vec: {:?}", rope_9_tail_vec);
            }

            // t_locs.insert((tx, ty));
            // println!("{} {}: H=({},{}), T=({},{})", dir, dist, hx, hy, tx, ty);
        }
    }

    // let ans1 = t_locs.into_iter().len() as i32;
    let ans1 = rope_tail_locs.into_iter().len() as i32;

    // println!(
    //     "orig: {}, rope: {}",
    //     ans1,
    //     rope_tail_locs.into_iter().len() as i32
    // );
    let ans2 = rope_9_tail_locs.into_iter().len() as i32;
    (ans1, ans2)
}

pub(crate) fn day_09(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_09_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 09: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_09_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/09.txt");
        let (ans1, ans2) = day_09_both_parts(&file_contents);
        assert_eq!(ans1, 13);
        if ans1 == 13 {
            println!("PASSED ans1 TEST");
        }
        assert_eq!(ans2, 1);
    }

    #[test]
    fn unit_test_2() {
        let file_contents = util::get_file_contents("test_data/09_02.txt");
        let (_, ans2) = day_09_both_parts(&file_contents);
        assert_eq!(ans2, 36);
    }
}

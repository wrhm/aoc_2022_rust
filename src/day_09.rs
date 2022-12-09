use crate::util;

use std::{collections::HashSet, time::Instant};

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
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;

    let mut dhx = 0;
    let mut dhy = 0;

    let mut t_locs: HashSet<(i32, i32)> = HashSet::new();
    t_locs.insert((0, 0));
    for (dir, dist) in commands {
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

            println!(
                "T=({},{}). H moving from ({},{}) to ({},{})",
                tx,
                ty,
                hx,
                hy,
                hx + dhx,
                hy + dhy
            );
            hx += dhx;
            hy += dhy;

            // horizontal
            if dhy == 0 && ty == hy && ((hx - tx) as i32).abs() >= 2 {
                println!("Moving T horizontally");
                tx += (hx - tx) / 2;
            }
            // vertical
            else if dhx == 0 && tx == hx && ((hy - ty) as i32).abs() >= 2 {
                println!("Moving T vertically");
                ty += (hy - ty) / 2;
            }
            // move T diagonally
            else if ((hx - tx) as i32).abs() >= 2 {
                println!("Moving T diagonally since xs had gap");
                tx = (hx + tx) / 2;
                ty = hy;
            } else if ((hy - ty) as i32).abs() >= 2 {
                println!("Moving T diagonally since ys had gap");
                ty = (hy + ty) / 2;
                tx = hx;
            }
            t_locs.insert((tx, ty));
            println!("{} {}: H=({},{}), T=({},{})", dir, dist, hx, hy, tx, ty);
        }
    }

    let ans1 = t_locs.into_iter().len() as i32;
    let ans2 = 0;
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
        let (ans1, _) = day_09_both_parts(&file_contents);
        assert_eq!(ans1, 13);
        // assert_eq!(ans2, -1);
    }
}

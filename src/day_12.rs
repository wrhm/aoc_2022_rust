use crate::util;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

// You'd like to reach E, but to save energy, you should do it in as few steps as possible. During each step, you can move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation of the destination square can be at most one higher than the elevation of your current square; that is, if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower than the elevation of your current square.)

fn day_12_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();
    let h = lines.len();
    let w = lines[0].len();
    let mut sr: i32 = -1;
    let mut sc: i32 = -1;
    let mut er: i32 = -1;
    let mut ec: i32 = -1;
    let mut hm: HashMap<(usize, usize), char> = HashMap::new();
    for (r, x) in lines.into_iter().enumerate() {
        for (c, ch) in x.chars().enumerate() {
            // hm[(r, c)] = ch
            hm.insert((r, c), ch);
            if ch == 'S' {
                sr = r as i32;
                sc = c as i32;
            } else if ch == 'E' {
                er = r as i32;
                ec = c as i32;
            }
        }
    }

    let mut ans1 = -1;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
    // let mut HashSet<(usize)>
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    q.push_back((sr as usize, sc as usize, 0));
    while !q.is_empty() {
        // println!("q: {:?}", q);
        let (r, c, dist) = q.pop_front().unwrap();
        // println!("Inspecting ({},{})", r, c);
        if seen.contains(&(r, c)) {
            continue;
        }
        seen.insert((r, c));
        if r as i32 == er && c as i32 == ec {
            println!("Found E at ({},{}) at step #{}", r, c, dist);
            ans1 = dist;
            // let mut pos = (r, c);
            // while parent.contains_key(&pos) {
            //     let next = *parent.get(&pos).unwrap();
            //     println!(
            //         "{:?} ({}) came from {:?} ({})",
            //         pos,
            //         hm.get(&pos).unwrap(),
            //         next,
            //         hm.get(&next).unwrap()
            //     );
            //     pos = next;
            // }
            break;
        }
        fn can_reach(ch_from: char, ch_to: char) -> bool {
            (ch_from == 'S' && (ch_to == 'a' || ch_to == 'b'))
                || (ch_to == 'E' && (ch_from == 'y' || ch_from == 'z'))
                || (ch_to != 'E' && (ch_to as i32 - ch_from as i32) <= 1)
            // || ((ch_to as i32) < (ch_from as i32))
        }

        let chcurr = *hm.get(&(r, c)).unwrap();
        if r >= 1 && hm.contains_key(&(r - 1, c)) {
            let chnxt = *hm.get(&(r - 1, c)).unwrap();
            // if chnxt == 'E' || (chnxt as i32 - chcurr as i32) <= 1 {
            if can_reach(chcurr, chnxt) && !seen.contains(&(r - 1, c)) {
                // println!("A");
                parent.insert((r - 1, c), (r, c));
                q.push_back((r - 1, c, dist + 1));
            } else {
                // println!("B");
            }
        }
        if r < h - 1 && hm.contains_key(&(r + 1, c)) {
            let chnxt = *hm.get(&(r + 1, c)).unwrap();
            // if chnxt == 'E' || (chnxt as i32 - chcurr as i32) <= 1 {
            if can_reach(chcurr, chnxt) && !seen.contains(&(r + 1, c)) {
                // println!("C");
                parent.insert((r + 1, c), (r, c));
                q.push_back((r + 1, c, dist + 1));
            } else {
                // println!("D. chnxt was {}", chnxt);
            }
        }
        if c >= 1 && hm.contains_key(&(r, c - 1)) {
            let chnxt = *hm.get(&(r, c - 1)).unwrap();
            // if chnxt == 'E' || (chnxt as i32 - chcurr as i32) <= 1 {
            if can_reach(chcurr, chnxt) && !seen.contains(&(r, c - 1)) {
                // println!("E");
                parent.insert((r, c - 1), (r, c));
                q.push_back((r, c - 1, dist + 1));
            } else {
                // println!("F");
            }
        }
        if c < w - 1 && hm.contains_key(&(r, c + 1)) {
            let chnxt = *hm.get(&(r, c + 1)).unwrap();
            // if chnxt == 'E' || (chnxt as i32 - chcurr as i32) <= 1 {
            if can_reach(chcurr, chnxt) && !seen.contains(&(r, c + 1)) {
                // println!("G");

                parent.insert((r, c + 1), (r, c));
                q.push_back((r, c + 1, dist + 1));
            } else {
                // println!("H. chnxt was {}", chnxt);
            }
        }
    }
    let mut pos = (er as usize, ec as usize);
    while parent.contains_key(&pos) {
        let next = *parent.get(&pos).unwrap();
        // println!(
        //     "{:?} ({}) came from {:?} ({})",
        //     pos,
        //     hm.get(&pos).unwrap(),
        //     next,
        //     hm.get(&next).unwrap()
        // );
        pos = next;
    }

    let ans2 = 0;
    (ans1, ans2)
}

pub(crate) fn day_12(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_12_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 12: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_12_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/12.txt");
        let (ans1, _) = day_12_both_parts(&file_contents);
        assert_eq!(ans1, 31);
        // assert_eq!(ans2, -1);
    }
}

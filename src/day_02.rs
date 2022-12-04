use crate::util;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Instant;

#[derive(PartialEq, Eq, Hash)]
enum RPS {
    R, // rock
    P, // paper
    S, // scissors
}
fn beats(a: &RPS, b: &RPS) -> bool {
    (a, b) == (&RPS::R, &RPS::S) || (a, b) == (&RPS::S, &RPS::P) || (a, b) == (&RPS::P, &RPS::R)
}
fn ties(a: &RPS, b: &RPS) -> bool {
    a == b
}

#[derive(PartialEq, Eq, Hash)]
enum Goal {
    Lose,
    Draw,
    Win,
}

lazy_static! {
    static ref FORCED_RESPONSE: HashMap<(&'static RPS, &'static Goal), &'static RPS> = {
        let mut map = HashMap::new();
        map.insert((&RPS::R, &Goal::Lose), &RPS::S);
        map.insert((&RPS::R, &Goal::Draw), &RPS::R);
        map.insert((&RPS::R, &Goal::Win), &RPS::P);
        map.insert((&RPS::P, &Goal::Lose), &RPS::R);
        map.insert((&RPS::P, &Goal::Draw), &RPS::P);
        map.insert((&RPS::P, &Goal::Win), &RPS::S);
        map.insert((&RPS::S, &Goal::Lose), &RPS::P);
        map.insert((&RPS::S, &Goal::Draw), &RPS::S);
        map.insert((&RPS::S, &Goal::Win), &RPS::R);
        map
    };
}

fn day_02_impl(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    // parse the two letters from each line of the file
    let mut pairs: Vec<(char, char)> = vec![];
    for line in &lines {
        if line.len() < 3 {
            continue;
        }
        let char_vec: Vec<char> = line.chars().collect();
        pairs.push((char_vec[0], char_vec[2]));
    }

    let xyz_score = HashMap::from([(RPS::R, 1), (RPS::P, 2), (RPS::S, 3)]);

    // part 1 interpretation (RPS, RPS)
    let rps_abc = HashMap::from([('A', RPS::R), ('B', RPS::P), ('C', RPS::S)]);
    let rps_xyz = HashMap::from([('X', RPS::R), ('Y', RPS::P), ('Z', RPS::S)]);

    let mut total_score = 0;
    for (them, you) in &pairs {
        let them_rps = rps_abc.get(them).unwrap();
        let you_rps = rps_xyz.get(you).unwrap();
        let mut score: i32 = *xyz_score.get(you_rps).unwrap();
        if beats(you_rps, them_rps) {
            score += 6;
        } else if ties(you_rps, them_rps) {
            score += 3;
        }
        total_score += score;
    }

    // part 2 interpretation (RPS, Goal)
    let xyz_goal = HashMap::from([('X', Goal::Lose), ('Y', Goal::Draw), ('Z', Goal::Win)]);

    let mut total_score2 = 0;
    for (them, you) in &pairs {
        let them_rps = rps_abc.get(them).unwrap();
        let goal = xyz_goal.get(you).unwrap();
        let resp = FORCED_RESPONSE.get(&(them_rps, goal)).unwrap();
        let mut score = *xyz_score.get(resp).unwrap();
        if goal == &Goal::Win {
            score += 6;
        } else if goal == &Goal::Draw {
            score += 3;
        }
        total_score2 += score;
    }
    let ans1 = total_score;
    let ans2 = total_score2;
    (ans1, ans2)
}

pub(crate) fn day_02(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_02_impl(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 02: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_02_impl;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/02.txt");
        let (ans1, ans2) = day_02_impl(&file_contents);
        assert_eq!(ans1, 15);
        assert_eq!(ans2, 12);
    }
}

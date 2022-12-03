use crate::util;

use std::collections::HashMap;
use std::time::Instant;

pub(crate) fn day_02() {
    let now = Instant::now();
    let s = util::get_file_contents("data/02.txt");
    let lines: Vec<&str> = s.split('\n').collect();

    // parse the two letters from each line of the file
    let mut pairs: Vec<(char, char)> = vec![];
    for line in &lines {
        if line.len() < 3 {
            continue;
        }
        pairs.push((line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()));
    }

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

    let xyz_score = HashMap::from([(RPS::R, 1), (RPS::P, 2), (RPS::S, 3)]);

    // part 1 interpretation (RPS, RPS)
    let rps_abc = HashMap::from([('A', RPS::R), ('B', RPS::P), ('C', RPS::S)]);
    let rps_xyz = HashMap::from([('X', RPS::R), ('Y', RPS::P), ('Z', RPS::S)]);

    let mut total_score = 0;
    for (them, you) in &pairs {
        let them_rps = rps_abc.get(&them).unwrap();
        let you_rps = rps_xyz.get(&you).unwrap();
        let mut score: i32 = *xyz_score.get(you_rps).unwrap();
        if beats(you_rps, them_rps) {
            score += 6;
        } else if ties(you_rps, them_rps) {
            score += 3;
        }
        total_score += score;
    }

    #[derive(PartialEq, Eq, Hash)]
    enum Goal {
        LOSE,
        DRAW,
        WIN,
    }

    // part 2 interpretation (RPS, Goal)
    let xyz_goal = HashMap::from([('X', Goal::LOSE), ('Y', Goal::DRAW), ('Z', Goal::WIN)]);
    let mut response: HashMap<(&RPS, &Goal), &RPS> = HashMap::new();
    response.insert((&RPS::R, &Goal::LOSE), &RPS::S);
    response.insert((&RPS::R, &Goal::DRAW), &RPS::R);
    response.insert((&RPS::R, &Goal::WIN), &RPS::P);
    response.insert((&RPS::P, &Goal::LOSE), &RPS::R);
    response.insert((&RPS::P, &Goal::DRAW), &RPS::P);
    response.insert((&RPS::P, &Goal::WIN), &RPS::S);
    response.insert((&RPS::S, &Goal::LOSE), &RPS::P);
    response.insert((&RPS::S, &Goal::DRAW), &RPS::S);
    response.insert((&RPS::S, &Goal::WIN), &RPS::R);

    let mut total_score2 = 0;
    for (them, you) in &pairs {
        let them_rps = rps_abc.get(&them).unwrap();
        let goal = xyz_goal.get(&you).unwrap();
        let resp = *response.get(&(them_rps, goal)).unwrap();
        let mut score = *xyz_score.get(resp).unwrap();
        if goal == &Goal::WIN {
            score += 6;
        } else if goal == &Goal::DRAW {
            score += 3;
        }
        total_score2 += score;
    }

    let ans1 = total_score;
    let ans2 = total_score2;
    let elapsed = now.elapsed();
    println!("Day 02: {}, {}. {:?}", ans1, ans2, elapsed);
}

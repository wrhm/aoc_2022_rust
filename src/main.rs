use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

fn get_file_contents(filename: &str) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why),
        Ok(_) => (),
    }
    s
}

fn day_01() {
    let now = Instant::now();
    let s = get_file_contents("data/01.txt");
    let lines: Vec<&str> = s.split('\n').collect();

    let mut vecs: Vec<Vec<i32>> = vec![];
    let mut vec: Vec<i32> = vec![];
    for line in &lines {
        let val_or = line.parse::<i32>();
        if val_or.is_ok() {
            vec.push(val_or.unwrap());
        } else {
            vecs.push(vec);
            vec = vec![];
        }
    }
    if !vec.is_empty() {
        vecs.push(vec);
    }
    let mut sums: Vec<i32> = vec![];
    for x in &vecs {
        sums.push(x.iter().sum());
    }
    // sort decreasing
    sums.sort_by(|a, b| b.cmp(a));
    let ans1 = sums.iter().max().unwrap();
    let ans2 = sums[0] + sums[1] + sums[2];
    let elapsed = now.elapsed();
    println!("Day 01: {}, {}. {:?}", ans1, ans2, elapsed);
}

fn day_02() {
    let now = Instant::now();
    let s = get_file_contents("data/02.txt");
    let lines: Vec<&str> = s.split('\n').collect();
    let mut pairs: Vec<(char, char)> = vec![];
    for line in &lines {
        if line.len() < 3 {
            continue;
        }
        pairs.push((line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()));
    }

    #[derive(PartialEq, Eq, Hash)]
    enum RPS {
        R,
        P,
        S,
    }
    fn beats(a: &RPS, b: &RPS) -> bool {
        (a, b) == (&RPS::R, &RPS::S) || (a, b) == (&RPS::S, &RPS::P) || (a, b) == (&RPS::P, &RPS::R)
    }
    fn ties(a: &RPS, b: &RPS) -> bool {
        a == b
    }
    let rps_abc = HashMap::from([('A', RPS::R), ('B', RPS::P), ('C', RPS::S)]);
    let rps_xyz = HashMap::from([('X', RPS::R), ('Y', RPS::P), ('Z', RPS::S)]);
    let xyz_score = HashMap::from([(RPS::R, 1), (RPS::P, 2), (RPS::S, 3)]);

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

fn main() {
    day_01();
    day_02();
}

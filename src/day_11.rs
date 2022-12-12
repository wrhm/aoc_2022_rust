use crate::util;

use std::collections::VecDeque;
use std::time::Instant;

type Items = VecDeque<i32>;
type RuleExpr<'a> = (char, &'a str);

fn parse_monkeys(lines: Vec<&str>) -> Vec<(Items, RuleExpr, i32, i32, i32)> {
    // (items,rule parts,modulus,true,false)
    let mut monkeys: Vec<(Items, RuleExpr, i32, i32, i32)> = vec![];
    let mut items: VecDeque<i32> = VecDeque::new();
    let mut_ref_items = &mut items;
    let mut rule_op: char = '_';
    let mut rule_val: &str = "";
    let mut modulus: i32 = 0;
    let mut true_ind: i32 = 0;
    let mut false_ind: i32;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().into_iter().collect();

        if parts[0] == "Monkey" {
            continue;
        } else if parts[0] == "Starting" {
            for p in parts.clone().into_iter() {
                if p.chars().next().unwrap().is_ascii_digit() {
                    mut_ref_items.push_back(p.replace(&[','][..], "").parse::<i32>().unwrap());
                }
            }
        } else if parts[0] == "Operation:" {
            rule_op = parts[4].chars().next().unwrap();
            rule_val = parts[5];
        } else if parts[0] == "Test:" {
            modulus = parts[3].parse::<i32>().unwrap();
        } else if parts[1] == "true:" {
            true_ind = parts.last().unwrap().parse::<i32>().unwrap();
        } else if parts[1] == "false:" {
            false_ind = parts.last().unwrap().parse::<i32>().unwrap();
            let monkey = (
                mut_ref_items.to_owned(),
                (rule_op, rule_val),
                modulus,
                true_ind,
                false_ind,
            );
            // println!("{:?}", monkey);
            monkeys.push(monkey);
            mut_ref_items.clear();
        }
    }
    monkeys
}

fn update_worry(item: i32, op: char, val: &str) -> i32 {
    let other = if val == "old" {
        item
    } else {
        val.parse::<i32>().unwrap()
    };
    if op == '+' {
        item + other
    } else {
        item * other
    }
}

fn day_11_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    let mut monkeys = parse_monkeys(lines);

    for m in &monkeys {
        println!("{:?}", m);
    }
    let mut inspections: Vec<i32> = (0..monkeys.len()).map(|_| 0).collect();

    for round in 1..(20 + 1) {
        for i in 0..monkeys.len() {
            let mut new_vals: Vec<(usize, i32)> = vec![];
            // begin first mutable scope
            {
                let (q, (op, val), mo, tr, fa) = &mut monkeys[i];
                while !q.is_empty() {
                    inspections[i] += 1;
                    let item = q.pop_front().unwrap();
                    let worry = update_worry(item, *op, val) / 3;
                    let recipient = if worry % *mo == 0 {
                        *tr as usize
                    } else {
                        *fa as usize
                    };
                    println!(
                        "Moving item from {} to {} as {} -> {} -> {}",
                        i, recipient, item, worry, worry
                    );
                    new_vals.push((recipient, worry));
                }
            }
            // begin (implict) second mutable scope
            for (j, v) in new_vals {
                println!("Throwing item {} to monkey {}", v, j);
                monkeys[j].0.push_back(v);
            }
            // println!("{:?}", monkeys);
        }
        println!("After round #{}:", round);
        for m in &monkeys {
            println!("{:?}", m.0);
        }
    }
    println!("inspections: {:?}", inspections);
    inspections.sort_by(|a, b| b.cmp(a));
    println!("inspections: {:?}", inspections);

    let ans1 = inspections[0] * inspections[1];
    let ans2 = 0;
    (ans1, ans2)
}

pub(crate) fn day_11(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_11_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 11: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_11_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/11.txt");
        let (ans1, _) = day_11_both_parts(&file_contents);
        assert_eq!(ans1, 10605);
        // assert_eq!(ans2, -1);
    }
}

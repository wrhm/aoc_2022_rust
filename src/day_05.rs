use crate::util;

fn day_05_impl(file_contents: &str) -> (String, String) {
    let lines: Vec<&str> = file_contents.split('\n').collect();

    // find number of columns
    let mut ncol: i32 = 0;
    for line in &lines {
        if line.contains('[') {
            continue;
        }
        if line.contains(" 1 ") {
            ncol = line
                .split_whitespace()
                .into_iter()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            break;
        }
    }
    println!("ncol: {}", ncol);

    let mut stacks: Vec<Vec<char>> = vec![];
    // include an unused stack #0 to reduce off-by-one errors
    for _ in 0..ncol + 1 {
        stacks.push(vec![]);
    }

    // push crates onto stacks
    for line in &lines {
        if line.contains(" 1 ") {
            break;
        }
        // letters are at (0-indexed) positions: 1,5,9,...
        for (i, c) in line.chars().enumerate() {
            if i % 4 != 1 || c == ' ' {
                continue;
            }
            let stack_ind = (i - 1) / 4 + 1;
            stacks[stack_ind].push(c);
        }
    }

    for i in 0..ncol + 1 {
        // stacks.push(vec![]);
        stacks[i as usize].reverse();
    }

    // for s in &stacks {
    //     println!("stack: {:?}", s);
    // }

    // handle moves
    for line in &lines {
        if !line.contains("move") {
            continue;
        }
        let nums: Vec<usize> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();
        let n = nums[0];
        let src = nums[1];
        let dest = nums[2];
        for _ in 0..n {
            let c = stacks[src].pop().unwrap();
            // println!("Moving {} from {} to {}", c, src, dest);
            stacks[dest].push(c);
            // println!("{:?}", stacks);
        }
    }

    let ans1: String = stacks[1..].iter().map(|x| x.last().unwrap()).collect();
    let ans2 = "".to_string();
    (ans1, ans2)
}

pub(crate) fn day_05(filename: &str) {
    util::day_n_str_str(filename, "05", day_05_impl);
}

#[cfg(test)]
mod tests {
    use super::day_05_impl;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/05.txt");
        let (ans1, ans2) = day_05_impl(&file_contents);
        assert_eq!(ans1, "CMZ".to_string());
        assert_eq!(ans2, "MCD".to_string());
    }
}

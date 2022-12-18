use crate::util;

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

use Packet::List;
use Packet::Num;

type Interval = (usize, usize);

fn parse_tokens(tokens: &Vec<&str>) -> Packet {
    // start by using a stack to find all sublist intervals
    let mut stack: Vec<usize> = vec![];
    let mut ranges: HashSet<Interval> = HashSet::new();
    for (i, t) in tokens.iter().enumerate() {
        if *t == "[" {
            stack.push(i);
        } else if *t == "]" {
            let f = stack.pop().unwrap();
            ranges.insert((f, i)); // sublist
        } else {
            ranges.insert((i, i)); // singleton (num)
        }
    }
    let mut ranges_v: Vec<Interval> = ranges.into_iter().collect();
    // sort first by range size, then by left edge
    ranges_v.sort_by(|a, b| (a.1 - a.0).cmp(&(b.1 - b.0)).then(a.0.cmp(&b.0)));

    let mut children: HashMap<Interval, Vec<Interval>> = HashMap::new();

    for (a, b) in &ranges_v {
        for (c, d) in &ranges_v {
            if a >= c && b <= d {
                // a,b is within c,d
                let ch = (*a, *b);
                let pa = (*c, *d);
                if ch == pa {
                    continue;
                }
                if let std::collections::hash_map::Entry::Vacant(e) = children.entry(pa) {
                    e.insert(vec![ch]);
                } else {
                    children.get_mut(&pa).unwrap().push(ch);
                }
                break;
            }
        }
    }
    build_packet((0, tokens.len() - 1), &children, tokens)
}

// initial call should be the entire interval. Recurses on children down to
// singleton `Num`s.
fn build_packet(
    intv: Interval,
    children: &HashMap<Interval, Vec<Interval>>,
    tokens: &Vec<&str>,
) -> Packet {
    if !children.contains_key(&intv) {
        if intv.0 == intv.1 {
            return Num(tokens[intv.0].parse::<i32>().unwrap() as i32);
        } else {
            return List(vec![]);
        }
    }
    let mut ps: Vec<Packet> = vec![];
    let mut ch_so: Vec<Interval> = vec![];
    for x in &children[&intv] {
        ch_so.push(*x);
    }
    ch_so.sort_by(|a, b| a.1.cmp(&b.1));
    // Traverse children sorted by left edge
    for c in ch_so {
        ps.push(build_packet(c, children, tokens));
    }
    List(ps)
}

fn parse_str_into_packet(line: &str) -> Packet {
    let mut parsed = line.replace(',', " ");
    parsed = parsed.replace('[', " < ");
    parsed = parsed.replace(']', " > ");
    parsed = parsed.replace('<', "[");
    parsed = parsed.replace('>', "]");
    let tokens: Vec<&str> = parsed.split_whitespace().collect();
    parse_tokens(&tokens)
}

fn cmp_packets(p: &Packet, q: &Packet) -> Option<std::cmp::Ordering> {
    match (p, q) {
        (Num(x), Num(y)) => Some(x.cmp(y)),
        (List(v), List(w)) => {
            let vlen = v.len();
            let wlen = w.len();

            for i in 0..std::cmp::min(vlen, wlen) {
                if i >= vlen {
                    // Reached end of left list
                    return Some(Less);
                } else if i >= wlen {
                    // Reached end of right list
                    return Some(Greater);
                } else {
                    let rec = cmp_packets(&v[i], &w[i])?;
                    if rec != Equal {
                        return Some(rec);
                    }
                }
            }
            Some(vlen.cmp(&wlen))
        }
        (Num(x), List(_)) => {
            let xmod = List(vec![Num(*x)]);
            cmp_packets(&xmod, q)
        }
        (List(_), Num(y)) => {
            let ymod = List(vec![Num(*y)]);
            cmp_packets(p, &ymod)
        }
    }
}

fn day_13_both_parts(file_contents: &str) -> (i32, i32) {
    let lines: Vec<&str> = file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect();
    let iter = lines.chunks(2);
    let mut ans1 = 0;
    for (i, ch) in iter.enumerate() {
        let (fst, snd) = (ch[0], ch[1]);
        let p = parse_str_into_packet(fst);
        let q = parse_str_into_packet(snd);
        let pq_cmp = cmp_packets(&p, &q);
        if pq_cmp.unwrap() == Less {
            ans1 += i as i32 + 1;
        }
    }

    let mut ans2 = 1;
    let mut packets: Vec<Packet> = lines.into_iter().map(parse_str_into_packet).collect();
    let p_two = List(vec![List(vec![Num(2)])]);
    let p_two_clone = List(vec![List(vec![Num(2)])]);
    let p_six = List(vec![List(vec![Num(6)])]);
    let p_six_clone = List(vec![List(vec![Num(6)])]);
    packets.push(p_two);
    packets.push(p_six);
    packets.sort_by(|a, b| cmp_packets(a, b).unwrap());
    for (i, p) in packets.iter().enumerate() {
        if cmp_packets(p, &p_two_clone) == Some(Equal) {
            ans2 *= i as i32 + 1;
        }
        if cmp_packets(p, &p_six_clone) == Some(Equal) {
            ans2 *= i as i32 + 1;
        }
    }
    (ans1, ans2)
}

pub(crate) fn day_13(filename: &str) {
    let now = Instant::now();
    let file_contents = util::get_file_contents(filename);
    let (ans1, ans2) = day_13_both_parts(&file_contents);
    let elapsed = now.elapsed();
    println!("Day 13: {}, {}. {:?}", ans1, ans2, elapsed);
}

#[cfg(test)]
mod tests {
    use super::day_13_both_parts;
    use crate::util;

    #[test]
    fn unit_test() {
        let file_contents = util::get_file_contents("test_data/13.txt");
        let (ans1, ans2) = day_13_both_parts(&file_contents);
        assert_eq!(ans1, 13);
        assert_eq!(ans2, 140);
    }
}

use crate::util;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

// struct Packet {

// }

#[derive(Debug)]
enum Packet {
    // EmptyList,
    Num(i32),
    List(Vec<Packet>),
}

use Packet::List;
use Packet::Num;

// fn parse_tokens(tokens: &Vec<&str>) -> Vec<Packet> {
//     println!("\ninspecting {:?}", tokens);
//     let mut p: Packet = Packet::List(vec![]);

//     if tokens.is_empty() {
//         return vec![Packet::List(vec![])];
//     }

//     if tokens[0] == "[" {
//         // find matching closing brace
//         let mut ci: i32 = -1;
//         let mut depth = 0;
//         for (i, s) in tokens.iter().enumerate() {
//             if *s == "[" {
//                 depth += 1;
//             } else if *s == "]" {
//                 depth -= 1;
//                 if depth == 0 {
//                     ci = i as i32;
//                     break;
//                 }
//             }
//         }
//         if ci == -1 {
//             panic!("couldn't find closing brace for {:?}", tokens);
//         }
//         println!("ci={}", ci);
//         // includes braces at each end
//         let v: &[&str] = &tokens[0..(1 + ci as usize)];
//         println!("found wrapped sublist at 0..{}: {:?}", 1 + ci, v);
//         let sub_v: &[&str] = &tokens[1..(ci as usize)];
//         println!("proper sublist at 1..{}: {:?}", ci, sub_v);
//         let sub_v_as_vec = sub_v.to_vec();
//         let mut rhs = &tokens[(1 + ci as usize)..(tokens.len())].to_vec();
//         println!("LHS: {:?}, RHS: {:?}", sub_v_as_vec, rhs);
//         let mut rec = parse_tokens(&sub_v_as_vec);
//         // return Packet::List(vec![rec]);

//         // rec.append(parse_tokens(mut &tokens[(ci as usize)..].to_vec()));
//         if !rhs.is_empty() {
//             rec.append(&mut parse_tokens(&mut rhs));
//         }
//         // return rec
//         return rec;
//     } else {
//         // panic!("Expected \"[\", saw \"{}\"", tokens[0]);
//         // return Packet::Num(tokens[0].parse::<i32>().unwrap());
//         let mut v: Vec<Packet> = vec![];
//         println!(
//             "about to parse {} as i32, as part of {:?}",
//             tokens[0], tokens
//         );
//         v.push(Packet::Num(tokens[0].parse::<i32>().unwrap()));

//         // v.append(parse_tokens(tokens(1..)));
//         if !tokens[1..].is_empty() {
//             v.append(&mut parse_tokens(&tokens[1..].to_vec()));
//         }
//         v
//     }
//     // Packet::List(vec![])
// }

// fn parse_tokens2(tokens: &Vec<&str>) -> Packet {
//     println!("\ninspecting {:?}", tokens);
//     // let mut p: Packet = Packet::List(vec![]);

//     if tokens.is_empty() {
//         return Packet::List(vec![]);
//     }

//     if tokens[0] == "[" {
//         // find matching closing brace
//         let mut ci: i32 = -1;
//         let mut depth = 0;
//         for (i, s) in tokens.iter().enumerate() {
//             if *s == "[" {
//                 depth += 1;
//             } else if *s == "]" {
//                 depth -= 1;
//                 if depth == 0 {
//                     ci = i as i32;
//                     break;
//                 }
//             }
//         }
//         if ci == -1 {
//             panic!("couldn't find closing brace for {:?}", tokens);
//         }
//         println!("ci={}", ci);
//         // includes braces at each end
//         let v: &[&str] = &tokens[0..(1 + ci as usize)];
//         println!("found wrapped sublist at 0..{}: {:?}", 1 + ci, v);
//         let sub_v: &[&str] = &tokens[1..(ci as usize)];
//         println!("proper sublist at 1..{}: {:?}", ci, sub_v);
//         let sub_v_as_vec = sub_v.to_vec();
//         let rhs = &tokens[(1 + ci as usize)..(tokens.len())].to_vec();
//         println!("LHS: {:?}, RHS: {:?}", sub_v_as_vec, rhs);
//         // let mut rec = parse_tokens2(&sub_v_as_vec);

//         let mut rec_L = parse_tokens2(&sub_v_as_vec);
//         if rhs.is_empty() {
//             return Packet::List(vec![rec_L]);
//         }
//         let mut rec_R = parse_tokens2(rhs);
//         Packet::List(vec![rec_L, rec_R])
//     } else {
//         let mut v: Vec<Packet> = vec![];
//         println!(
//             "about to parse {} as i32, as part of {:?}",
//             tokens[0], tokens
//         );
//         let first = Packet::Num(tokens[0].parse::<i32>().unwrap());
//         v.push(first);

//         if !tokens[1..].is_empty() {
//             // v.append(&mut parse_tokens2(&tokens[1..].to_vec()));
//             let mut rest = parse_tokens2(&tokens[1..].to_vec());
//             let mut unpacked = match rest {
//                 Packet::Num(n) => vec![rest],
//                 Packet::List(vp) => vp,
//             };
//             v.append(&mut unpacked);
//         }
//         if v.len() == 1 {
//             return Packet::Num(tokens[0].parse::<i32>().unwrap());
//         }
//         Packet::List(v)
//         // Packet::Num(tokens[0].parse::<i32>().unwrap())
//         // Packet::Num(tokens.into_iter().map(|t| t.parse::<i32>().unwrap()))
//     }
//     // Packet::List(vec![])
// }

// fn parse_tokens3(tokens: &Vec<&str>) -> Packet {
//     println!("\ninspecting {:?}", tokens);
//     if tokens.is_empty() {
//         return Packet::List(vec![]);
//         // panic!("empty list");
//     }
//     // // if tokens[0] == "[" && tokens[1] == "]" {
//     // //     return Packet::List(vec![]);
//     // // }

//     // if tokens[0] == "[" {
//     //     // find matching closing brace
//     //     let mut ci: i32 = -1;
//     //     let mut depth = 0;
//     //     for (i, s) in tokens.iter().enumerate() {
//     //         if *s == "[" {
//     //             depth += 1;
//     //         } else if *s == "]" {
//     //             depth -= 1;
//     //             if depth == 0 {
//     //                 ci = i as i32;
//     //                 break;
//     //             }
//     //         }
//     //     }
//     //     if ci == -1 {
//     //         panic!("couldn't find closing brace for {:?}", tokens);
//     //     }
//     //     println!("ci={}", ci);
//     //     // includes braces at each end
//     //     let v: &[&str] = &tokens[0..(1 + ci as usize)];
//     //     // println!("found wrapped sublist at 0..{}: {:?}", 1 + ci, v);
//     //     let sub_v: &[&str] = &tokens[1..(ci as usize)];
//     //     // println!("proper sublist at 1..{}: {:?}", ci, sub_v);
//     //     let sub_v_as_vec = sub_v.to_vec();
//     //     let rhs = &tokens[(1 + ci as usize)..(tokens.len())].to_vec();
//     //     println!("LHS: {:?}, RHS: {:?}", sub_v_as_vec, rhs);
//     //     // if rhs.is_empty() {
//     //     //     return Packet::List(vec![parse_tokens3(&sub_v_as_vec)]);
//     //     // } else {

//     //     // }
//     //     // return Packet::List(parse_tokens3(&sub_v_as_vec));
//     // } else {
//     //     return Packet::Num(tokens[0].parse::<i32>().unwrap());
//     // }

//     // // stack-based
//     // let mut p: Packet = Packet::List(vec![]);
//     // // let stack:Vec<&str> = vec![];
//     // let stack: Vec<Packet> = vec![];
//     // for &t in tokens {

//     // }
//     // let mut stack: Vec<&str> = vec![];
//     // for &t in tokens {
//     //     if t == "]" {
//     //         let mut v: Vec<&str> = vec!["]"];
//     //         while *v.last().unwrap() != "[" {
//     //             v.push(stack.pop().unwrap());
//     //         }
//     //         v.reverse();
//     //         println!("subitem: {:?}", v);
//     //     } else {
//     //         stack.push(t);
//     //     }
//     // }

//     Num(0)
// }

type Interval = (usize, usize);

fn parse_tokens4(tokens: &Vec<&str>) -> Packet {
    // if start == end {
    //     Num(tokens[start].parse::<i32>().unwrap())
    // }

    // new idea: identify bounds of all sublists, then toposort bounds enclosed by others
    let mut stack: Vec<usize> = vec![];
    let mut ranges: HashSet<Interval> = HashSet::new();
    let mut parent: HashMap<Interval, Interval> = HashMap::new();
    // let mut children: HashMap<Interval, HashSet<Interval>> = HashMap::new();
    let mut children: HashMap<Interval, Vec<Interval>> = HashMap::new();
    for (i, t) in tokens.iter().enumerate() {
        if *t == "[" {
            stack.push(i);
        } else if *t == "]" {
            let f = stack.pop().unwrap();
            ranges.insert((f, i));
            // println!("sublist from {} to {}", f, i);
        } else {
            ranges.insert((i, i));
            // println!("singleton number at {}", i);
        }
    }
    let mut ranges_v: Vec<Interval> = ranges.into_iter().collect();
    // ranges_v.sort_by(|a, b| (a.1 - a.0).cmp(&(b.1 - b.0)));

    // sort first by range size, then by left edge
    ranges_v.sort_by(|a, b| (a.1 - a.0).cmp(&(b.1 - b.0)).then(a.0.cmp(&b.0)));
    println!("rv {:?}", ranges_v);

    for (a, b) in &ranges_v {
        for (c, d) in &ranges_v {
            if a >= c && b <= d {
                // a,b is within c,d
                let ch = (*a, *b);
                let pa = (*c, *d);
                if ch == pa {
                    continue;
                }
                parent.insert(ch, pa);
                if let std::collections::hash_map::Entry::Vacant(e) = children.entry(pa) {
                    // let mut hs: HashSet<Interval> = HashSet::new();
                    // hs.insert(ch);
                    // e.insert(hs);
                    e.insert(vec![ch]);
                } else {
                    children.get_mut(&pa).unwrap().push(ch);
                }
                break;
            }
        }
    }
    println!("pa {:?}", parent);
    println!("ch {:?}", children);
    let ret = build_packet((0, tokens.len() - 1), &children, tokens);
    println!("bp {:?}", ret);
    ret
}

fn build_packet(
    intv: Interval,
    children: &HashMap<Interval, Vec<Interval>>,
    tokens: &Vec<&str>,
) -> Packet {
    // if !children.contains_key(&intv) {
    //     if intv.0 == intv.1 {
    //         return Num(tokens[intv.0].parse::<i32>().unwrap() as i32);
    //     } else {
    //         return List(vec![]);
    //     }
    // }

    // if intv.0 == intv.1 {
    //     if !children.contains_key(&intv) {
    //         // println!("creating Num for {}", tokens[intv.0]);
    //         return Num(tokens[intv.0].parse::<i32>().unwrap() as i32);
    //     } else {
    //         return List(vec![]);
    //     }
    // }

    if !children.contains_key(&intv) {
        if intv.0 == intv.1 {
            // println!("creating Num for {}", tokens[intv.0]);
            return Num(tokens[intv.0].parse::<i32>().unwrap() as i32);
        } else {
            return List(vec![]);
        }
    }

    let mut ps: Vec<Packet> = vec![];
    // let ch_so = &children[&intv].sort_by(|a, b| a.1.cmp(&b.1));
    // let mut ch_so = &children[&intv].clone();
    let mut ch_so: Vec<Interval> = vec![];
    for x in &children[&intv] {
        ch_so.push(*x);
    }
    ch_so.sort_by(|a, b| a.1.cmp(&b.1));
    // Traverse children sorted by left.
    // for c in &children[&intv] {
    for c in ch_so {
        // println!("{:?} is a child of {:?}", c, intv);
        ps.push(build_packet(c, children, tokens));
    }
    // ps.reverse();
    List(ps)
}

fn parse_str_into_packet(line: &str) -> Packet {
    let mut parsed = line.replace(',', " ");
    parsed = parsed.replace('[', " < ");
    parsed = parsed.replace(']', " > ");
    parsed = parsed.replace('<', "[");
    parsed = parsed.replace('>', "]");
    let tokens: Vec<&str> = parsed.split_whitespace().collect();
    // let tokens: Vec<&str> = parsed.split_whitespace().take(5).collect();
    println!("\n=====\ntokens {:?}", tokens);
    // parse_tokens(&tokens)
    // let p = parse_tokens(&tokens);
    // println!("parsed whole packet as {:?}", p);
    // let p = parse_tokens3(&tokens);
    let p = parse_tokens4(&tokens); //, 0, tokens.len() - 1);
    println!("parsed whole packet as {:?}", p);
    // p[0]
    // Packet::Num(0)
    p
}

use std::cmp::Ordering::{Equal, Greater, Less};

// fn cmp_packets(p: &Packet, q: &Packet) -> std::cmp::Ordering {
//     println!("Comparing {:?} and {:?}", p, q);
//     match (p, q) {
//         (Num(x), Num(y)) => {
//             println!("Comparing Nums directly");
//             x.cmp(y)
//         }
//         (List(v), List(w)) => {
//             let vlen = v.len();
//             let wlen = w.len();
//             println!(
//                 "Comparing {:?} (len {}) and {:?} (len {})",
//                 v, vlen, w, wlen
//             );
//             for i in 0..std::cmp::min(vlen, wlen) {
//                 if i >= vlen {
//                     println!("Reached end of left list");
//                     return Less;
//                 } else if i >= wlen {
//                     println!("Reached end of right list");
//                     return Greater;
//                 } else {
//                     let rec = cmp_packets(&v[i], &w[i]);
//                     if rec != Equal {
//                         return rec;
//                     }
//                     // return cmp_packets(&v[i], &w[i]);
//                 }
//             }

//             if vlen < wlen {
//                 println!("Left list is empty");
//                 Less
//             } else {
//                 println!("Right list is empty");
//                 Greater
//             }
//             // Equal
//         }
//         (Num(x), List(_)) => {
//             // cmp_packets(&List(vec![Num(*x)]), List(w))
//             let xmod = List(vec![Num(*x)]);
//             // let orig_w = List(*w);
//             // cmp_packets(&xmod, &orig_w)
//             cmp_packets(&xmod, q)
//         }
//         //  TODO: handle mixed-type cases
//         // _ => {
//         //     println!("TODO: handle mixed-type cases");
//         //     Equal
//         // }
//         (List(_), Num(y)) => {
//             let ymod = List(vec![Num(*y)]);
//             //   let orig_v = List(*v);
//             cmp_packets(p, &ymod)
//         }
//     }
// }

fn cmp_packets2(p: &Packet, q: &Packet) -> Option<std::cmp::Ordering> {
    println!("Comparing Packets {:?} and {:?}", p, q);
    match (p, q) {
        (Num(x), Num(y)) => {
            println!("Comparing Nums directly: {} vs {}", x, y);
            Some(x.cmp(y))
        }
        (List(v), List(w)) => {
            let vlen = v.len();
            let wlen = w.len();
            println!(
                "Comparing Lists {:?} (len {}) and {:?} (len {})",
                v, vlen, w, wlen
            );

            for i in 0..std::cmp::min(vlen, wlen) {
                println!("Planning to compare {:?} with {:?}", v[i], w[i]);
            }
            for i in 0..std::cmp::min(vlen, wlen) {
                if i >= vlen {
                    println!("Reached end of left list");
                    return Some(Less);
                } else if i >= wlen {
                    println!("Reached end of right list");
                    return Some(Greater);
                } else {
                    let rec = cmp_packets2(&v[i], &w[i])?;
                    if rec != Equal {
                        return Some(rec);
                    }
                    // return cmp_packets2(&v[i], &w[i]);
                }
            }
            Some(vlen.cmp(&wlen))
        }
        (Num(x), List(_)) => {
            // cmp_packets(&List(vec![Num(*x)]), List(w))
            let xmod = List(vec![Num(*x)]);
            // let orig_w = List(*w);
            // cmp_packets(&xmod, &orig_w)
            cmp_packets2(&xmod, q)
        }
        (List(_), Num(y)) => {
            let ymod = List(vec![Num(*y)]);
            //   let orig_v = List(*v);
            cmp_packets2(p, &ymod)
        }
    }
}

fn day_13_both_parts(file_contents: &str) -> (i32, i32) {
    // let lines: Vec<&str> = file_contents.split('\n').collect();

    // let mut santized: Vec<Vec<&str>> = vec![];
    // let mut santized: Vec<Vec<String>> = vec![];
    // for line in lines {
    //     if line.is_empty() {
    //         continue;
    //     }
    //     // let parsed: Vec<&str> = line
    //     //     .replace(",", " ")
    //     //     .replace("[", "< ")
    //     //     .replace("]", "> ")
    //     //     .replace("<", "[")
    //     //     .replace(">", "]")
    //     //     .split_whitespace()
    //     //     .collect();

    //     // let mut parsed = line.replace(',', " ");
    //     // parsed = parsed.replace('[', " < ");
    //     // parsed = parsed.replace(']', " > ");
    //     // parsed = parsed.replace('<', "[");
    //     // parsed = parsed.replace('>', "]");
    //     // let parts: Vec<&str> = parsed.split_whitespace().collect();
    //     // println!("{:?}", parts);
    //     let p = parse_str_into_packet(line);
    //     println!("{:?}", p);

    //     // let parts_as_strings: Vec<String> =
    //     //     parsed.split_whitespace().map(|x| x.to_string()).collect();
    //     // santized.push(parts_as_strings);
    //     // santized.push(parsed.split_whitespace().collect());
    //     // println!("{:?}", santized);
    // }

    let lines: Vec<&str> = file_contents
        .split('\n')
        .filter(|x| !x.is_empty())
        // .take(4) // TODO: remove .take
        .collect();
    let iter = lines.chunks(2);
    let mut ans1 = 0;
    for (i, ch) in iter.enumerate() {
        let (fst, snd) = (ch[0], ch[1]);
        let p = parse_str_into_packet(fst);
        let q = parse_str_into_packet(snd);
        // let pq_cmp = cmp_packets(&p, &q);
        let pq_cmp = cmp_packets2(&p, &q);
        // if pq_cmp == None {
        //     println!("Comparison errored");
        // } else
        if pq_cmp.unwrap() == Less {
            println!("Decision: Less (right order)");
            ans1 += i as i32 + 1;
        } else if pq_cmp.unwrap() == Equal {
            println!("Decision: Equal");
        } else {
            println!("Decision: Greater (wrong order)");
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
    packets.sort_by(|a, b| cmp_packets2(a, b).unwrap());
    println!("Sorted:");
    // println!("{:?}", packets);
    for (i, p) in packets.iter().enumerate() {
        println!("{}: {:?}", i + 1, p);
        if cmp_packets2(p, &p_two_clone) == Some(Equal) {
            println!("^FOUND 2 at {}", i + 1);
            // break;
            ans2 *= i as i32 + 1;
        }
        if cmp_packets2(p, &p_six_clone) == Some(Equal) {
            println!("^FOUND 6 at {}", i + 1);
            // break;
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

#![feature(extract_if)]
#![feature(slice_split_once)]
#![feature(test)]

use std::collections::HashMap;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Clone)]
struct Rule<'a> {
    jump_to: &'a [u8],
    variable: u8,
    condition: u8,
    num: u64,
}

fn process(data: &str) -> u64 {
    let data = data.as_bytes();
    let mut workflows = HashMap::new();

    for workflow in data.split(|&x| x == b'\n') {
        if workflow.is_empty() {
            break;
        }
        let (name, remain) = workflow.split_once(|&x| x == b'{').unwrap();

        let mut rules: Vec<Rule> = vec![];
        for branch in remain.split(|&x| x == b',') {
            let rule = if branch.contains(&b'<') || branch.contains(&b'>') {
                let variable = branch[0];
                let condition = branch[1];
                let num: String = branch[2..]
                    .iter()
                    .map(|&x| x as char)
                    .take_while(|&x| x.is_ascii_digit())
                    .collect();
                let l = num.len();
                let num: u64 = num.parse::<u64>().unwrap();
                let jump_to = &branch[3 + l..branch.len()];

                Rule {
                    jump_to,
                    variable,
                    condition,
                    num,
                }
            } else {
                let jump_to = &branch[0..branch.len() - 1];

                Rule {
                    jump_to,
                    condition: 0,
                    num: 0,
                    variable: 0,
                }
            };
            rules.push(rule);
        }

        workflows.insert(name, rules);
    }

    let mut answer = 0;
    let mut set = Vec::new();
    let start = vec![b'i', b'n'];

    set.push((start.as_slice(), 1..4001, 1..4001, 1..4001, 1..4001));

    while let Some((wname, mut x, mut m, mut a, mut s)) = set.pop() {
        let rules = workflows.get(wname.as_ref()).unwrap();

        for rule in rules {
            let (x, m, a, s) = match (rule.variable, rule.condition) {
                (b'x', b'>') if x.contains(&rule.num) => {
                    let new_x = rule.num + 1..x.end;
                    x = x.start..rule.num + 1;

                    (new_x, m.clone(), a.clone(), s.clone())
                }
                (b'x', b'<') if x.contains(&rule.num) => {
                    let new_x = x.start..rule.num;
                    x = rule.num..x.end;

                    (new_x, m.clone(), a.clone(), s.clone())
                }
                (b'm', b'>') if m.contains(&rule.num) => {
                    let new_m = rule.num + 1..m.end;
                    m = m.start..rule.num + 1;

                    (x.clone(), new_m, a.clone(), s.clone())
                }
                (b'm', b'<') if m.contains(&rule.num) => {
                    let new_m = m.start..rule.num;
                    m = rule.num..m.end;

                    (x.clone(), new_m, a.clone(), s.clone())
                }
                (b'a', b'>') if a.contains(&rule.num) => {
                    let new_a = rule.num + 1..a.end;
                    a = a.start..rule.num + 1;

                    (x.clone(), m.clone(), new_a, s.clone())
                }
                (b'a', b'<') if a.contains(&rule.num) => {
                    let new_a = a.start..rule.num;
                    a = rule.num..a.end;

                    (x.clone(), m.clone(), new_a, s.clone())
                }
                (b's', b'>') if s.contains(&rule.num) => {
                    let new_s = rule.num + 1..s.end;
                    s = s.start..rule.num + 1;

                    (x.clone(), m.clone(), a.clone(), new_s)
                }
                (b's', b'<') if s.contains(&rule.num) => {
                    let new_s = s.start..rule.num;
                    s = rule.num..s.end;

                    (x.clone(), m.clone(), a.clone(), new_s)
                }
                (0, 0) => (x.clone(), m.clone(), a.clone(), s.clone()),
                _ => unreachable!(),
            };

            if rule.jump_to == [b'R'] {
                continue;
            }
            if rule.jump_to == [b'A'] {
                answer +=
                    (x.end - x.start) * (m.end - m.start) * (a.end - a.start) * (s.end - s.start);
                continue;
            }
            set.push((&rule.jump_to, x, m, a, s));
        }
    }

    answer
}

fn main() {
    for input in INPUTS.iter() {
        println!("answer = {}", process(input));
    }
}

#[bench]
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

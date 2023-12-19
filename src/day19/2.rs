#![feature(extract_if)]
#![feature(slice_split_once)]
#![feature(test)]

use std::collections::HashMap;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
struct Rule {
    jump_to: String,
    variable: char,
    condition: char,
    num: u64,
}

fn process(data: &str) -> u64 {
    let mut data = data.split("\n\n");

    let mut workflows = HashMap::new();

    for workflow in data.next().unwrap().lines() {
        let (name, remain) = workflow.split_once(|x| x == '{').unwrap();
        let mut rules = vec![];
        for branch in remain.split(',') {
            let rule = if branch.contains(|x| x == '<' || x == '>') {
                let branch: Vec<char> = branch.chars().collect();
                let variable = branch[0];
                let condition = branch[1];
                let num: String = branch[2..]
                    .iter()
                    .take_while(|&x| x.is_ascii_digit())
                    .collect();
                let num: u64 = num.parse::<u64>().unwrap();

                let mut jump_to = String::new();
                for &c in branch.iter().rev() {
                    if c == '}' {
                        continue;
                    }

                    if c == ':' {
                        break;
                    }

                    jump_to.push(c);
                }

                let jump_to: String = jump_to.chars().rev().collect();

                Rule {
                    jump_to,
                    variable,
                    condition,
                    num,
                }
            } else {
                let branch: Vec<char> = branch.chars().collect();

                let mut jump_to = String::new();
                for &c in branch.iter().rev() {
                    if c == '}' {
                        continue;
                    }

                    if c == ':' {
                        break;
                    }

                    jump_to.push(c);
                }

                let jump_to: String = jump_to.chars().rev().collect();

                Rule {
                    jump_to,
                    condition: ' ',
                    num: 0,
                    variable: ' ',
                }
            };
            rules.push(rule);
        }

        workflows.insert(name, rules);
    }

    let mut answer = 0;

    let mut set = Vec::new();
    set.push(("in".to_string(), 1..=4000, 1..=4000, 1..=4000, 1..=4000));

    while !set.is_empty() {
        let mut next = Vec::new();

        for set_ip in set.drain(..) {
            let (ref wname, mut x, mut m, mut a, mut s) = set_ip.clone();
            let rules = workflows.get(wname.as_str()).unwrap();

            for rule in rules {
                match (rule.variable, rule.condition) {
                    ('x', '>') if x.contains(&rule.num) => {
                        let new_x = rule.num + 1..=*x.end();

                        let remain_x = *x.start()..=rule.num;
                        x = remain_x;

                        if !new_x.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                new_x,
                                m.clone(),
                                a.clone(),
                                s.clone(),
                            ));
                        }
                    }
                    ('x', '<') if x.contains(&rule.num) => {
                        let new_x = *x.start()..=rule.num - 1;
                        x = rule.num..=*x.end();
                        if !new_x.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                new_x,
                                m.clone(),
                                a.clone(),
                                s.clone(),
                            ));
                        }
                    }
                    ('m', '>') if m.contains(&rule.num) => {
                        let new_m = rule.num + 1..=*m.end();
                        m = *m.start()..=rule.num;

                        if !new_m.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                x.clone(),
                                new_m,
                                a.clone(),
                                s.clone(),
                            ));
                        }
                    }
                    ('m', '<') if m.contains(&rule.num) => {
                        let new_m = *m.start()..=rule.num - 1;
                        m = rule.num..=*m.end();

                        if !new_m.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                x.clone(),
                                new_m,
                                a.clone(),
                                s.clone(),
                            ));
                        }
                    }
                    ('a', '>') if a.contains(&rule.num) => {
                        let new_a = rule.num + 1..=*a.end();
                        a = *a.start()..=rule.num;
                        if !new_a.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                x.clone(),
                                m.clone(),
                                new_a,
                                s.clone(),
                            ));
                        }
                    }
                    ('a', '<') if a.contains(&rule.num) => {
                        let new_a = *a.start()..=rule.num - 1;
                        a = rule.num..=*a.end();

                        if !new_a.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                x.clone(),
                                m.clone(),
                                new_a,
                                s.clone(),
                            ));
                        }
                    }
                    ('s', '<') if s.contains(&rule.num) => {
                        let new_s = *s.start()..=rule.num - 1;
                        s = rule.num..=*s.end();

                        if !new_s.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                x.clone(),
                                m.clone(),
                                a.clone(),
                                new_s,
                            ));
                        }
                    }
                    ('s', '>') if s.contains(&rule.num) => {
                        let new_s = rule.num + 1..=*s.end();
                        s = *s.start()..=rule.num;
                        if !new_s.is_empty() {
                            next.push((
                                rule.jump_to.clone(),
                                x.clone(),
                                m.clone(),
                                a.clone(),
                                new_s,
                            ));
                        }
                    }
                    (' ', ' ') => {
                        next.push((
                            rule.jump_to.clone(),
                            x.clone(),
                            m.clone(),
                            a.clone(),
                            s.clone(),
                        ));
                    }
                    _ => unreachable!(),
                }
            }
        }

        next.retain(|(wname, _, _, _, _)| wname != "R");

        for (_, x, m, a, s) in next.extract_if(|(wname, _, _, _, _)| wname == "A") {
            answer += (x.end() - x.start() + 1)
                * (m.end() - m.start() + 1)
                * (a.end() - a.start() + 1)
                * (s.end() - s.start() + 1);
        }

        set = next;
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

#![feature(slice_split_once)]
#![feature(test)]

use std::collections::{hash_map::Keys, HashMap};
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
struct Rule {
    variable: Option<char>,
    condition: Option<char>,
    match_against: Option<i64>,
    jump_to: String,
}

fn process(data: &str) -> i64 {
    let mut answer = 0;

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
                let num: i64 = num.parse::<i64>().unwrap();

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
                    variable: Some(variable),
                    condition: Some(condition),
                    match_against: Some(num),
                    jump_to,
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
                    variable: None,
                    condition: None,
                    match_against: None,
                    jump_to,
                }
            };
            rules.push(rule);
        }

        workflows.insert(name, rules);
    }

    for input in data.next().unwrap().lines() {
        let input: Vec<char> = input
            .chars()
            .filter(|&x| x.is_ascii_alphabetic() || x.is_ascii_digit() || x == ',')
            .collect();

        let mut vars = HashMap::new();

        for set in input.split(|&x| x == ',') {
            let variable = set[0];
            let value: String = set[1..].iter().collect();
            let value: i64 = value.parse::<i64>().unwrap();

            vars.insert(variable, value);
        }

        let mut wname = "in";

        let mut accepted = false;

        loop {
            if wname == "A" {
                accepted = true;
                break;
            } else if wname == "R" {
                accepted = false;
                break;
            }

            let rules = workflows.get(&wname).unwrap();

            for rule in rules {
                if let Some(variable) = rule.variable {
                    let var_value = *vars.get(&variable).unwrap();
                    let check_against = rule.match_against.unwrap();

                    let result = match rule.condition.unwrap() {
                        '>' => var_value > check_against,
                        '<' => var_value < check_against,
                        _ => unreachable!(),
                    };

                    if result {
                        wname = &rule.jump_to;
                        break;
                    }
                } else {
                    wname = &rule.jump_to;
                    break;
                }
            }
        }

        if accepted {
            for v in vars.values() {
                answer += v;
            }
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
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

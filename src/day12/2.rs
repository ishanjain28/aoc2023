#![feature(iter_intersperse)]
#![feature(test)]

extern crate test;

use std::collections::HashMap;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;

    for line in data.lines() {
        let (springs, count) = line.split_once(' ').unwrap();

        // 5 ss
        // 4 interspersed string
        let springs: Vec<char> = std::iter::repeat(springs)
            .intersperse("?")
            .take(9)
            .flat_map(|x| x.chars().collect::<Vec<char>>())
            .collect();

        let count: Vec<usize> = count
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let l = count.len();
        let count: Vec<usize> = count.into_iter().cycle().take(5 * l).collect();

        let total: usize = count.iter().sum();

        let mut active = vec![];

        let mut map = HashMap::new();

        answer += combination_helper(
            &springs,
            &mut map,
            &count,
            &mut active,
            0,
            0,
            0,
            0,
            total,
            springs.len(),
        );
    }

    answer
}

fn combination_helper(
    expected: &[char],
    map: &mut HashMap<(usize, usize, usize, usize), usize>,
    groups: &[usize],
    active: &mut Vec<char>,
    e_idx: usize,
    mut g_idx: usize,
    inserted: usize,
    continugous_length: usize,
    target: usize,
    max_length: usize,
) -> usize {
    if active.len() >= max_length && inserted != target {
        return 0;
    }

    if target - inserted > max_length - active.len() {
        return 0;
    }

    if continugous_length == 0 && active.len() >= 2 && active[active.len() - 2] == '#' {
        g_idx += 1;
    }

    if inserted == target && active.len() == max_length {
        return 1;
    }

    if let Some(v) = map.get(&(e_idx, g_idx, inserted, continugous_length)) {
        return *v;
    }
    let mut sum = 0;
    match expected[e_idx] {
        '#' if g_idx < groups.len() && continugous_length < groups[g_idx] => {
            active.push('#');
            sum += combination_helper(
                expected,
                map,
                groups,
                active,
                e_idx + 1,
                g_idx,
                inserted + 1,
                continugous_length + 1,
                target,
                max_length,
            );
            active.pop();
        }

        '.' => {
            active.push('.');
            sum += combination_helper(
                expected,
                map,
                groups,
                active,
                e_idx + 1,
                g_idx,
                inserted,
                0,
                target,
                max_length,
            );
            active.pop();
        }

        '?' => {
            if g_idx < groups.len() && continugous_length < groups[g_idx] {
                active.push('#');
                sum += combination_helper(
                    expected,
                    map,
                    groups,
                    active,
                    e_idx + 1,
                    g_idx,
                    inserted + 1,
                    continugous_length + 1,
                    target,
                    max_length,
                );
                active.pop();
            }
            active.push('.');
            sum += combination_helper(
                expected,
                map,
                groups,
                active,
                e_idx + 1,
                g_idx,
                inserted,
                0,
                target,
                max_length,
            );
            active.pop();
        }

        _ => (),
    }

    map.insert((e_idx, g_idx, inserted, continugous_length), sum);
    sum
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

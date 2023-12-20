#![feature(slice_split_once)]
#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::collections::{HashMap, VecDeque};
extern crate test;

const INPUTS: [&str; 1] = [include_str!("./input.txt")];

#[inline]
fn compress(mut c: &[u8]) -> usize {
    if c.len() > 3 {
        c = &c[..3];
    }
    let mut out = 0;
    let mut i = c.len() as i32 - 1;

    let mut pow = 1;
    while i >= 0 {
        out += pow * (c[i as usize] - b'a') as usize;

        pow *= 26;

        i -= 1;
    }

    out
}

fn process(data: &str) -> usize {
    let mut map = HashMap::new();
    let mut conjunction_inputs = HashMap::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let (src, remain) = line.split_once(' ').unwrap();

        let src: Vec<char> = src.chars().collect();

        let (stype, label) = match src[0] {
            '%' => (src[0], (&src[1..])),
            '&' => (src[0], &src[1..]),
            'b' => (' ', src.as_slice()),

            _ => unreachable!(),
        };

        let remain = &remain[3..];

        let dst: Vec<String> = remain
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.to_string())
            .collect();

        let label: String = label.iter().collect();

        map.insert(label, (stype, dst));
    }

    for (k, (ntype, dst)) in map.iter() {
        let ntype = *ntype;
        if ntype != '&' && ntype != '%' {
            continue;
        }
        for dst in dst {
            if let Some(&(dtype, _)) = map.get(&dst.to_string()) {
                if dtype == '&' {
                    conjunction_inputs
                        .entry(dst.to_string())
                        .or_insert_with(Vec::new)
                        .push(k.to_string());
                }
            }
        }
    }

    let mut button_state: HashMap<String, bool> = HashMap::new();

    for k in map.keys() {
        button_state.insert(k.clone(), false);
    }

    let mut button_presses = vec![];
    let mut button_press = 0;

    loop {
        button_press += 1;
        cycle(
            button_press,
            &mut button_state,
            &mut map,
            &mut conjunction_inputs,
            &mut button_presses,
        );

        if button_presses.len() == conjunction_inputs.get("zh").map_or(0, |x| x.len()) {
            break;
        }
    }

    lcm(&button_presses)
}

fn lcm(a: &[usize]) -> usize {
    let mut answer = a[0];

    for &num in a.iter().skip(1) {
        answer = (num * answer) / (gcd(num, answer));
    }

    answer
}

#[inline]
const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn cycle(
    button_press: usize,
    state: &mut HashMap<String, bool>,
    map: &mut HashMap<String, (char, Vec<String>)>,
    conjunction_inputs: &mut HashMap<String, Vec<String>>,
    button_presses: &mut Vec<usize>,
) {
    let mut q = VecDeque::new();

    let (_, broadcast_dst) = map.get(&"broadcaster".to_string()).unwrap();
    for dst in broadcast_dst {
        q.push_back((dst, false));
    }

    while let Some((dst, pulse)) = q.pop_front() {
        let (dtype, dnext) = match map.get(dst) {
            Some(v) => v,
            None => continue,
        };

        if pulse && dst == "zh" {
            button_presses.push(button_press);
        }

        match dtype {
            '%' => {
                if pulse {
                    continue;
                }

                if let Some(state) = state.get_mut(dst) {
                    *state = !*state;

                    for next in dnext.iter() {
                        q.push_back((next, *state));
                    }
                } else {
                    unreachable!()
                }
            }

            '&' => {
                let new_state = conjunction_inputs
                    .get(dst)
                    .unwrap()
                    .iter()
                    .all(|x| *state.get(x).unwrap());

                for next in dnext {
                    q.push_back((next, !new_state));
                }

                *state.get_mut(dst).unwrap() = !new_state;
            }
            _ => unreachable!(),
        }
    }
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

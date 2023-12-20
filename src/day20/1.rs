#![feature(slice_split_once)]
#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::collections::{HashMap, VecDeque};
extern crate test;

const INPUTS: [&str; 3] = [
    "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
    include_str!("./sample.txt"),
    include_str!("./input.txt"),
];

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

fn process(data: &str) -> i64 {
    let data = data.as_bytes();

    let mut map = vec![];

    let mut conjunction_inputs = HashMap::new();

    for line in data.split(|&x| x == b'\n') {
        if line.is_empty() {
            continue;
        }
        let (src, remain) = line.split_once(|&x| x == b' ').unwrap();

        let (stype, label) = match src[0] {
            b'%' => (src[0], compress(&src[1..])),
            b'&' => (src[0], compress(&src[1..])),
            b'b' => (0, compress(src)),

            _ => unreachable!(),
        };

        let remain = &remain[3..];

        let dst: Vec<usize> = remain
            .split(|&x| x == b',')
            .map(|x| x.trim_ascii())
            .map(compress)
            .collect();

        if map.len() <= label {
            map.extend(std::iter::repeat((0, vec![])).take(label - map.len() + 1))
        }

        map[label] = (stype, dst);
    }

    for (k, (ntype, dst)) in map.iter().enumerate() {
        let ntype = *ntype;
        if ntype != b'&' && ntype != b'%' {
            continue;
        }
        for &dst in dst {
            if let Some(&(dtype, _)) = map.get(dst) {
                if dtype == b'&' {
                    conjunction_inputs
                        .entry(dst)
                        .or_insert_with(Vec::new)
                        .push(k);
                }
            }
        }
    }

    let mut button_state = vec![false; map.len()];

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut q = VecDeque::new(); // Button pressed, send signal to broadcaster

        let (_, broadcast_dst) = &map[compress("broadcaster".as_bytes())]; //map.get(&compress("broadcaster".as_bytes())).unwrap();
                                                                           //
        low += 1;

        for &dst in broadcast_dst {
            q.push_back((dst, 0));
        }

        while let Some((node, pulse)) = q.pop_front() {
            if pulse == 0 {
                low += 1;
            } else {
                high += 1;
            }

            let (ntype, ndst) = if let Some(o) = map.get(node) {
                o
            } else {
                continue;
            };

            match *ntype {
                b'%' => {
                    if pulse == 1 {
                        continue;
                    }

                    if let Some(state) = button_state.get_mut(node) {
                        *state = !*state;
                        for &dst in ndst {
                            q.push_back((dst, if *state { 1 } else { 0 }));
                        }
                    } else {
                        unreachable!()
                    }
                }
                b'&' => {
                    // Update state

                    let new_state = conjunction_inputs
                        .get(&node)
                        .unwrap()
                        .iter()
                        .all(|&x| *button_state.get(x).unwrap());

                    for &dst in ndst {
                        q.push_back((dst, !new_state as i64));
                    }

                    *button_state.get_mut(node).unwrap() = !new_state;
                }

                0 => (),
                _ => unreachable!(),
            }
        }
    }

    low * high
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

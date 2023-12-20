#![feature(slice_split_once)]
#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::collections::VecDeque;
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
    let data = data.as_bytes();

    let mut map = vec![];
    let mut inverted_map = vec![];

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
            map.extend(std::iter::repeat((0, vec![])).take(label - map.len() + 1));
        }

        map[label] = (stype, dst);
    }

    inverted_map.extend(std::iter::repeat(vec![]).take(map.len() - inverted_map.len()));

    for (k, (_, dst)) in map.iter().enumerate() {
        for &dst in dst {
            inverted_map[dst].push(k);
        }
    }

    let mut button_state = vec![false; map.len() + 1];

    let mut button_presses = vec![];
    let mut button_press = 0;

    let rx_input = *inverted_map[compress("rx".as_bytes())]
        .iter()
        .next()
        .unwrap();

    loop {
        button_press += 1;
        cycle(
            button_press,
            &mut button_state,
            &map,
            &inverted_map,
            &mut button_presses,
            rx_input,
        );

        if button_presses.len() == inverted_map[rx_input].len() {
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
    state: &mut [bool],
    map: &[(u8, Vec<usize>)],
    inverted_map: &[Vec<usize>],
    button_presses: &mut Vec<usize>,
    rx_input: usize,
) {
    let mut q = VecDeque::new();

    let (_, broadcast_dst) = &map[compress("broadcaster".as_bytes())];
    for &dst in broadcast_dst {
        q.push_back((dst, false));
    }

    while let Some((dst, pulse)) = q.pop_front() {
        if pulse && dst == rx_input {
            button_presses.push(button_press);
            continue;
        }

        let (dtype, dnext) = &map[dst];
        if *dtype == 0 {
            continue;
        }
        match dtype {
            b'%' => {
                if pulse {
                    continue;
                }

                state[dst] = !state[dst];
                for &next in dnext.iter() {
                    q.push_back((next, state[dst]));
                }
            }

            b'&' => {
                let new_state = inverted_map[dst].iter().all(|&x| state[x]);

                for &next in dnext {
                    q.push_back((next, !new_state));
                }

                state[dst] = !new_state;
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
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

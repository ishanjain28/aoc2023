#![feature(slice_split_once)]
#![feature(test)]

use std::collections::HashMap;

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> usize {
    let (seq, remain) = data.split_once(|&x| x == b'\n').unwrap();

    let mut map = HashMap::new();

    for line in remain.split(|&x| x == b'\n').skip(1) {
        if line.is_empty() {
            continue;
        }
        let (start, remain) = line.split_at(3);
        let (l, r) = (&remain[4..7], &remain[9..12]);

        map.insert(start, (l, r));
    }

    let mut a_set = vec![];

    for &k in map
        .keys()
        .filter(|x| x.last().map_or(false, |&b| b == b'A'))
    {
        a_set.push(k);
    }

    let mut tmp = Vec::with_capacity(a_set.len());

    'outer: for mut node in a_set {
        for (i, ins) in seq.iter().cycle().enumerate() {
            if node.last().map_or(false, |&b| b == b'Z') {
                tmp.push(i);
                continue 'outer;
            }

            let (l, r) = map.get(node).unwrap();

            match ins {
                b'L' => node = l,
                b'R' => node = r,
                _ => (),
            }
        }
    }

    lcm(&tmp)
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

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[bench]
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

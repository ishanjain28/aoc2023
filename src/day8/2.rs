#![feature(slice_split_once)]
#![feature(test)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> usize {
    let (seq, remain) = data.split_once(|&x| x == b'\n').unwrap();

    let mut map = vec![];
    for line in remain.split(|&x| x == b'\n').skip(1) {
        if line.is_empty() {
            continue;
        }
        let (start, remain) = line.split_at(3);
        let (l, r) = (&remain[4..7], &remain[9..12]);

        // Pack data into 6 bytes each
        let start = lut(start[0]) << 10 | lut(start[1]) << 5 | lut(start[2]);
        let l = lut(l[0]) << 10 | lut(l[1]) << 5 | lut(l[2]);
        let r = lut(r[0]) << 10 | lut(r[1]) << 5 | lut(r[2]);

        if start >= map.len() {
            map.extend(std::iter::repeat(None).take(start - map.len() + 1));
        }

        map[start] = Some((l, r));
    }

    let mut set = vec![];

    const A: usize = lut(b'A');
    const Z: usize = lut(b'Z');

    for (i, _) in map.iter().enumerate().filter(|(_, v)| v.is_some()) {
        let last = i & 0b11_111;
        if last == A {
            set.push(i);
        }
    }

    let mut tmp = Vec::with_capacity(set.len());

    'outer: for mut node in set {
        for (i, ins) in seq.iter().cycle().enumerate() {
            let last = node & 0b11_111;
            if last == Z {
                tmp.push(i);
                continue 'outer;
            }

            let (l, r) = map[node].unwrap();

            match ins {
                b'L' => node = l,
                b'R' => node = r,
                _ => (),
            }
        }
    }

    lcm(&tmp)
}

#[inline]
const fn lut(c: u8) -> usize {
    match c {
        (b'0'..=b'9') => (c - b'0') as usize + 27,
        (b'A'..=b'Z') => (c - b'A') as usize,

        _ => unreachable!(),
    }
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

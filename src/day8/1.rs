#![feature(slice_split_once)]
#![feature(test)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    // RL
    //
    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)
    &[
        82, 76, 10, 10, 65, 65, 65, 32, 61, 32, 40, 66, 66, 66, 44, 32, 67, 67, 67, 41, 10, 66, 66,
        66, 32, 61, 32, 40, 68, 68, 68, 44, 32, 69, 69, 69, 41, 10, 67, 67, 67, 32, 61, 32, 40, 90,
        90, 90, 44, 32, 71, 71, 71, 41, 10, 68, 68, 68, 32, 61, 32, 40, 68, 68, 68, 44, 32, 68, 68,
        68, 41, 10, 69, 69, 69, 32, 61, 32, 40, 69, 69, 69, 44, 32, 69, 69, 69, 41, 10, 71, 71, 71,
        32, 61, 32, 40, 71, 71, 71, 44, 32, 71, 71, 71, 41, 10, 90, 90, 90, 32, 61, 32, 40, 90, 90,
        90, 44, 32, 90, 90, 90, 41,
    ],
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

    const AAA: usize = lut(b'A') << 10 | lut(b'A') << 5 | lut(b'A');
    const ZZZ: usize = lut(b'Z') << 10 | lut(b'Z') << 5 | lut(b'Z');

    let mut pos = AAA;

    for (i, ins) in seq.iter().cycle().enumerate() {
        if pos == ZZZ {
            return i;
        }

        let (l, r) = map[pos].unwrap();

        match ins {
            b'L' => pos = l,
            b'R' => pos = r,

            _ => (),
        }
    }

    0
}

#[inline]
const fn lut(c: u8) -> usize {
    match c {
        (b'0'..=b'9') => (c - b'0') as usize + 27,
        (b'A'..=b'Z') => (c - b'A') as usize,

        _ => unreachable!(),
    }
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

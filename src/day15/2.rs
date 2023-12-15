#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;

    let mut map = vec![vec![]; 256];

    for set in data.as_bytes().split(|&x| x == b',') {
        let op_idx = set.iter().position(|&x| x == b'=' || x == b'-').unwrap();
        let bbox = pack(&set[0..op_idx]);

        let op = set[op_idx];
        let box_id = hash(&set[0..op_idx]);

        match op {
            b'-' => {
                let vec = &mut map[box_id];

                if let Some(idx) = vec.iter().position(|(lbox, _)| lbox == &bbox) {
                    vec.remove(idx);
                }
            }

            b'=' => {
                let lens_focal_length = (set[op_idx + 1] - b'0') as usize;

                if let Some(idx) = map[box_id].iter().position(|(lbox, _)| lbox == &bbox) {
                    map[box_id][idx] = (bbox, lens_focal_length);
                } else {
                    map[box_id].push((bbox, lens_focal_length));
                }
            }

            _ => (),
        }
    }

    for (box_id, vec) in map.iter().enumerate() {
        for (j, (_, lens)) in vec.iter().enumerate() {
            answer += (1 + box_id) * (j + 1) * lens;
        }
    }

    answer
}

#[inline]
const fn pack(ip: &[u8]) -> u64 {
    debug_assert!(ip.len() <= 8);
    let mut out = 0;
    let mut i = 0;

    while i < ip.len() {
        out |= ip[i] as u64;
        out <<= 8;
        i += 1;
    }

    out
}

#[inline]
const fn hash(ip: &[u8]) -> usize {
    let mut current: usize = 0;
    let mut i = 0;
    while i < ip.len() {
        let c = ip[i];
        current += c as usize;
        current *= 17;
        current %= 256;
        i += 1;
    }
    current
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

#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;
    let (mut sx, mut sy) = (0, 0);
    let mut internal = 0;

    for line in data.lines() {
        let line = line.as_bytes();
        let l = line.len();

        let line = &line[l - 6 - 1..line.len() - 1];

        let dist = hex2int(&line[0..5]);

        let (x, y): (i64, i64) = match line[5] {
            b'0' => (0, 1),
            b'1' => (1, 0),
            b'2' => (0, -1),
            b'3' => (-1, 0),
            _ => unreachable!(),
        };

        internal += ((x * dist) + (y * dist)).abs();
        let (dx, dy) = (sx + (x * dist), sy + (y * dist));

        answer += sx * dy - dx * sy;
        sx = dx;
        sy = dy;
    }

    let answer = answer.abs();

    ((answer / 2) + 1) + (internal / 2)
}

#[inline]
const fn hex2int(ip: &[u8]) -> i64 {
    let mut out = 0;

    let mut i = 0;
    while i < ip.len() {
        let byte = match ip[i] {
            v @ b'0'..=b'9' => v - b'0',
            v @ b'a'..=b'f' => v - b'a' + 10,
            v @ b'A'..=b'F' => v - b'A' + 10,
            _ => unreachable!(),
        };

        out <<= 4;
        out |= byte as i64;

        i += 1;
    }

    out
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

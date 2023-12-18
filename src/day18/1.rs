#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;
    let mut internal = 0;

    let (mut sx, mut sy) = (0, 0);

    for line in data.lines() {
        let line = line.as_bytes();
        let l = line.len();

        let line = &line[0..l - 10];
        let dist = {
            let mut dist = 0;
            let mut power = 10i64.pow(line.len() as u32 - 3);
            for c in line[2..].iter() {
                dist += power * (c - b'0') as i64;
                power /= 10;
            }
            dist
        };

        let (x, y) = match line[0] {
            b'R' => (0, 1),
            b'D' => (1, 0),
            b'L' => (0, -1),
            b'U' => (-1, 0),
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

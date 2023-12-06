#![feature(test)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut data = data.split(|&x| x == b'\n');

    let time: f64 = data
        .next()
        .map(|line| {
            let mut num = 0;
            let mut mul = 1;
            for c in line.iter().filter(|c| c.is_ascii_digit()).rev() {
                num += (c - b'0') as u64 * mul;
                mul *= 10;
            }
            num as f64
        })
        .unwrap_or(0.0);
    let distance: f64 = data
        .next()
        .map(|line| {
            let mut num = 0;
            let mut mul = 1;
            for c in line.iter().filter(|c| c.is_ascii_digit()).rev() {
                num += (c - b'0') as u64 * mul;
                mul *= 10;
            }
            num as f64
        })
        .unwrap_or(0.0);

    let mut root1 = (time + (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
    let root2 = (time - (time.powi(2) - 4.0 * distance).sqrt()) / 2.0;
    if root1.fract() == 0.0 && root2.fract() == 0.0 {
        root1 -= 1.0;
    }

    root1 as u64 - root2 as u64
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

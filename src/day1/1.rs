#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

fn process(input: &[u8]) -> u32 {
    let mut total = 0;
    for line in input.split(|&x| x == b'\n') {
        let first = line
            .iter()
            .find(|c| c.is_ascii_digit())
            .map_or(0, |x| 10 * (x - b'0'));

        let last = line
            .iter()
            .rev()
            .find(|c| c.is_ascii_digit())
            .map_or(0, |x| x - b'0');

        total += first as u32 + last as u32;
    }

    total
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

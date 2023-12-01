#![feature(test)]
extern crate test;

const INPUTS: [&'static str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn main() {
    for input in INPUTS.iter() {
        let mut total = 0;
        for line in input.split('\n') {
            total += process(line);
        }

        println!("total = {}", total);
    }
}

fn process(data: &str) -> u32 {
    let first = data
        .chars()
        .find(|c| c.is_numeric())
        .map_or(0, |x| 10 * x.to_digit(10).unwrap());

    let last = data
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .map_or(0, |x| x.to_digit(10).unwrap());

    first + last
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

#![feature(test)]

use std::ops::Range;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut lines = data.split("\n\n");

    let seeds = lines
        .next()
        .map(|x| x.split(' ').skip(1).map(|y| y.parse::<i64>().unwrap()))
        .unwrap();
    let maps: Vec<Vec<(Range<i64>, Range<i64>)>> = lines
        .map(|lines| {
            lines
                .lines()
                .skip(1)
                .map(|line| {
                    let y: Vec<i64> = line
                        .split_ascii_whitespace()
                        .map(|y| y.parse::<i64>().unwrap())
                        .collect();
                    let size = y[2];

                    (y[1]..y[1] + size, y[0]..y[0] + size)
                })
                .collect()
        })
        .collect();

    let mut answer = std::i64::MAX;

    for mut src in seeds {
        for map in maps.iter() {
            let dst = map
                .iter()
                .find(|x| x.0.contains(&src))
                .map(|x| src + x.1.start - x.0.start)
                .unwrap_or(src);

            src = dst;
        }

        answer = std::cmp::min(answer, src);
    }

    answer
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

#![feature(test)]

use std::ops::RangeInclusive;

extern crate test;

const INPUTS: [&str; 1] = [include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;

    let mut stones = vec![];
    for line in data.lines() {
        let (a, b) = line.split_once(" @ ").unwrap();

        let mut pos = a
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap());

        let mut vel = b
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap());

        stones.push((
            (pos.next().unwrap(), pos.next().unwrap()),
            (vel.next().unwrap(), vel.next().unwrap()),
        ));
    }

    for i in 0..stones.len() {
        let a = stones[i];

        let x1 = a.0 .0;
        let y1 = a.0 .1;
        let x2 = a.0 .0 + a.1 .0;
        let y2 = a.0 .1 + a.1 .1;

        let m1 = (y2 - y1) as f64 / (x2 - x1) as f64;
        let c1 = y1 as f64 - m1 * x1 as f64;

        for b in stones.iter().skip(i + 1) {
            let x3 = b.0 .0;
            let y3 = b.0 .1;
            let x4 = b.0 .0 + b.1 .0;
            let y4 = b.0 .1 + b.1 .1;

            let m2 = (y4 - y3) as f64 / (x4 - x3) as f64;
            let c2 = y3 as f64 - m2 * x3 as f64;

            // parallel or coincidental
            if m1 == m2 {
                continue;
            }

            // y = m1 * x + c1;
            // y = m2 * x + c2;
            // x if y is equal
            // (c1 - c2) / (m1 - m2)
            // put this back in x and then y is
            // y = m1 * i_x + c1
            let i_x = (c2 - c1) / (m1 - m2);
            let i_y = m1 * i_x + c1;

            if (i_x - x1 as f64).signum() as i64 != a.1 .0.signum()
                || (i_x - x3 as f64).signum() as i64 != b.1 .0.signum()
            {
                continue;
            }

            const LOOKUP_RANGE: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;

            answer += (LOOKUP_RANGE.contains(&i_x) && LOOKUP_RANGE.contains(&i_y)) as i64
        }
    }

    answer
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

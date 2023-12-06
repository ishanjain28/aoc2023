#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut data = data.lines();

    let time: u64 = data
        .next()
        .map(|line| {
            let x: String = line.chars().filter(|c| c.is_numeric()).collect();

            x.parse::<u64>().unwrap()
        })
        .unwrap_or(0);
    let distance: u64 = data
        .next()
        .map(|line| {
            let x: String = line.chars().filter(|c| c.is_numeric()).collect();

            x.parse::<u64>().unwrap()
        })
        .unwrap_or(0);

    let mut answer = 0;

    for t in 0..time {
        let time_left = time - t;
        let speed = t;

        let dist_traveled = time_left * speed;

        if dist_traveled > distance {
            answer += 1;
        }
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

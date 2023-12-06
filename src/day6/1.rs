#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut data = data.lines();

    let time = data
        .next()
        .map(|line| {
            line.split(' ')
                .filter(|x| !x.is_empty())
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
        })
        .unwrap();

    let distance = data
        .next()
        .map(|line| {
            line.split(' ')
                .filter(|x| !x.is_empty())
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
        })
        .unwrap();

    let mut answer = 1;

    for (time, dist) in time.zip(distance) {
        let mut ways = 0;

        for t in 0..time {
            let time_left = time - t;
            let speed = t;

            let dist_traveled = time_left * speed;

            if dist_traveled > dist {
                ways += 1;
            }
        }

        answer *= ways;
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

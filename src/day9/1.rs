#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let mut input: Vec<i64> = line
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let mut diffs = vec![input.clone()];

        while input.iter().any(|&x| x != 0) {
            let diff: Vec<i64> = input.windows(2).map(|win| win[1] - win[0]).collect();
            input = diff.clone();
            diffs.push(diff);
        }

        let mut prev_inserted = 0;

        for diff in diffs.iter_mut().rev() {
            let last = diff[diff.len() - 1];
            let num = prev_inserted + last;
            diff.push(num);

            prev_inserted = num;
        }

        answer += diffs[0].last().unwrap();
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

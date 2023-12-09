#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let input: Vec<i64> = line.split(' ').flat_map(|x| x.parse::<i64>()).collect();

        let mut diffs = vec![input];

        while let Some(last) = diffs.last() {
            if !last.iter().any(|&x| x != 0) {
                break;
            }
            let diff: Vec<i64> = last.windows(2).map(|win| win[1] - win[0]).collect();

            diffs.push(diff);
        }

        let mut prev_inserted = 0;
        for diff in diffs.drain(..).rev() {
            let last = diff[diff.len() - 1];
            let num = prev_inserted + last;
            prev_inserted = num;
        }
        answer += prev_inserted;
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

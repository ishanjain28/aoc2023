#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::{cmp::Ordering, collections::HashMap};

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;

    let mut data = data.split("\n\n");

    let seq: Vec<char> = data.next().map(|x| x.chars().collect()).unwrap();

    let mut map = HashMap::new();

    for line in data.next().unwrap().lines() {
        let (start, remain) = line.split_once(" = ").unwrap();

        let (l, r) = remain.split_once(',').unwrap();
        let l: String = l.chars().filter(|x| x.is_ascii_alphabetic()).collect();
        let r: String = r.chars().filter(|x| x.is_ascii_alphabetic()).collect();

        map.insert(start, (l, r));
    }

    let mut i = 0;
    let mut pos = "AAA";

    loop {
        let step = seq[i % seq.len()];
        if pos == "ZZZ" {
            return answer;
        }

        let (l, r) = map.get(&pos).unwrap();

        println!("pos = {} l = {} r = {}", pos, l, r);
        match step {
            'R' => pos = r,
            'L' => pos = l,
            _ => unreachable!(),
        }

        answer += 1;

        i += 1;
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

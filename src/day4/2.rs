#![feature(test)]

use std::collections::HashSet;
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> u64 {
    let mut cards = vec![];

    let mut total = 0;
    for data in data.lines() {
        let (x, nums) = data.split_once(':').unwrap();

        let v = x.split_at(4).1.trim();
        let id = v.parse::<usize>().unwrap();

        let (nums, wins) = nums.split_once('|').unwrap();
        let nums: Vec<&str> = nums.split(' ').collect();
        let wins: Vec<&str> = wins.split(' ').collect();

        let mut card = vec![];
        let mut win = HashSet::new();

        for num in nums {
            if num.is_empty() {
                continue;
            }

            let num = num.parse::<u64>().unwrap();

            card.push(num);
        }

        for w in wins {
            if w.is_empty() {
                continue;
            }
            let num = w.parse::<u64>().unwrap();
            win.insert(num);
        }

        cards.push((id, card, win));
    }

    let mut stack: Vec<(usize, Vec<u64>, HashSet<u64>)> = cards.iter().rev().cloned().collect();

    let mut count = vec![0; cards.len() + 1];

    while let Some((id, card, wins)) = stack.pop() {
        let mut sum = 0;
        for c in card {
            if wins.contains(&c) {
                sum += 1;
            }
        }

        count[id] += 1;

        for i in id..id + sum {
            stack.push(cards[i].clone())
        }
    }

    for c in count.iter().skip(1) {
        total += c;
    }

    println!("{:?}", count);
    total
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

#![feature(slice_split_once)]
#![feature(test)]

use fxhash::FxHashSet;

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut win_num = FxHashSet::default();

    let mut total = 0;
    for data in data.split(|&x| x == b'\n') {
        if data.is_empty() {
            continue;
        }

        let (_, nums) = data.split_once(|&x| x == b':').unwrap();

        let (nums, wins) = nums.split_once(|&x| x == b'|').unwrap();
        let nums = nums.split(|&x| x == b' ');
        let wins = wins.split(|&x| x == b' ');

        for win in wins {
            if win.is_empty() {
                continue;
            }
            let win = parse(win);

            win_num.insert(win);
        }

        let mut val = 0;
        for num in nums {
            if num.is_empty() {
                continue;
            }
            let num = parse(num);
            if win_num.contains(&num) {
                if val == 0 {
                    val = 1;
                } else {
                    val *= 2;
                }
            }
        }

        total += val;

        win_num.clear();
    }

    total
}

#[inline]
fn parse(b: &[u8]) -> u64 {
    let mut out = 0;

    let mut pow = 1;
    for c in b.iter().rev() {
        out += (c - b'0') as u64 * pow;
        pow *= 10;
    }

    out
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

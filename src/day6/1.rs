#![feature(test)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut data = data.split(|&x| x == b'\n');

    let time = data
        .next()
        .map(|line| {
            line.split(|&x| x == b' ')
                .skip(1)
                .filter(|c| !c.is_empty())
                .map(|c| {
                    let mut num = 0;
                    let mut mul = 1;

                    for c in c.iter().rev() {
                        num += (c - b'0') as u64 * mul;
                        mul *= 10;
                    }

                    num as f32
                })
        })
        .unwrap();

    let distance = data
        .next()
        .map(|line| {
            line.split(|&x| x == b' ')
                .skip(1)
                .filter(|c| !c.is_empty())
                .map(|c| {
                    let mut num = 0;
                    let mut mul = 1;

                    for c in c.iter().rev() {
                        num += (c - b'0') as u64 * mul;
                        mul *= 10;
                    }

                    num as f32
                })
        })
        .unwrap();

    let mut answer = 1;

    for (time, dist) in time.zip(distance) {
        let mut root1 = (time + (time * time - 4.0 * dist).sqrt()) / 2.0;
        let root2 = (time - (time * time - 4.0 * dist).sqrt()) / 2.0;

        if root1.fract() == 0.0 && root2.fract() == 0.0 {
            root1 -= 1.0;
        }

        answer *= root1 as u64 - root2 as u64;
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

#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut total = 0;

    for line in data.split(|&x| x == b'\n') {
        if line.is_empty() {
            continue;
        }
        let line = &line[5..];
        let (gid, mut line) = read_int(line);
        let mut possible = true;

        while !line.is_empty() {
            let (num, l) = read_int(&line[1..]);

            match l {
                [b'r', ..] => {
                    line = &l[3..];
                    if num > 12 {
                        possible = false;
                    }
                }
                [b'g', ..] => {
                    line = &l[5..];
                    if num > 13 {
                        possible = false;
                    }
                }
                [b'b', ..] => {
                    line = &l[4..];
                    if num > 14 {
                        possible = false;
                    }
                }
                [b' ', ..] => line = l,
                _ => (),
            }
        }

        if possible {
            total += gid;
        }
    }

    total
}

fn read_int(line: &[u8]) -> (u64, &[u8]) {
    let mut num = 0u64;
    let mut multiplier = 10_000u64;
    let mut ops_done = 0;

    for i in 0..line.len() {
        let c = line[i];

        if c.is_ascii_digit() {
            let c = line[i] - b'0';
            ops_done += 1;
            num += multiplier * c as u64;
            multiplier /= 10;
        } else if num != 0 {
            return (num / (10u64.pow(5 - ops_done)), &line[i + 1..]);
        } else {
            break;
        }
    }
    (num / (10u64.pow(5 - ops_done)), line)
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

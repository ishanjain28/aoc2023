#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Red(u32),
    Green(u32),
    Blue(u32),
    Unknown,
}

fn process(data: &str) -> u64 {
    let mut total = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let (gid, remain) = line.split_once(':').unwrap();

        let gid = gid
            .split_once(' ')
            .map(|(_, x)| x.parse::<u64>().unwrap())
            .unwrap();

        let moves = remain.split(';').map(|x| {
            let mut output = 0u64;
            let cubes = x.split(',');

            for cube in cubes {
                let cube = cube.trim();
                let (count, color) = cube.split_once(' ').unwrap();

                let count = count.parse::<u64>().unwrap();

                match color {
                    "red" => output |= count << 32,
                    "green" => output |= count << 16,
                    "blue" => output |= count,
                    _ => unreachable!(),
                }
            }

            output
        });

        let mut possible = true;
        for mmove in moves {
            let red = (mmove & (0xffff << 32)) >> 32;
            let green = (mmove & (0xffff << 16)) >> 16;
            let blue = mmove & (0xffff);

            if red > 12 || green > 13 || blue > 14 {
                possible = false;
                break;
            }
        }
        if possible {
            total += gid;
        }
    }

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

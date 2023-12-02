#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
pub enum Move {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn process(data: &str) -> u32 {
    let mut total = 0;

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let (_, remain) = line.split_once(':').unwrap();

        let moves = remain.split(';').flat_map(|x| {
            let mut output = vec![];
            let cubes = x.split(',');

            for cube in cubes {
                let cube = cube.trim();
                let (count, color) = cube.split_once(' ').unwrap();

                let count = count.parse::<u32>().unwrap();

                match color {
                    "red" => output.push(Move::Red(count)),
                    "green" => output.push(Move::Green(count)),
                    "blue" => output.push(Move::Blue(count)),
                    _ => unreachable!(),
                }
            }

            output
        });

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for mmove in moves {
            match mmove {
                Move::Red(v) => min_red = std::cmp::max(min_red, v),
                Move::Green(v) => min_green = std::cmp::max(min_green, v),
                Move::Blue(v) => min_blue = std::cmp::max(min_blue, v),
            }
        }

        total += min_red * min_green * min_blue;
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

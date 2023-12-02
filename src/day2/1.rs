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
        let (gid, remain) = line.split_once(':').unwrap();

        let gid = gid
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let moves = remain.split(';').map(|x| {
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

        let mut possible = true;
        'outer: for mmove in moves {
            for step in mmove {
                match step {
                    Move::Red(v) if v > 12 => {
                        possible = false;
                        break 'outer;
                    }
                    Move::Green(v) if v > 13 => {
                        possible = false;
                        break 'outer;
                    }
                    Move::Blue(v) if v > 14 => {
                        possible = false;
                        break 'outer;
                    }

                    _ => (),
                }
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

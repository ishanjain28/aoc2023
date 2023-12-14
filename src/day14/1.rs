#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let size = data.lines().count();

    let data = data.as_bytes();
    let mut answer = 0;

    let mut available_space = [0; 100];

    for (i, line) in data.chunks_exact(size + 1).enumerate() {
        for (c, a) in line.iter().zip(available_space.iter_mut()) {
            match c {
                b'#' => *a = 0,
                b'.' => *a += 1,

                b'O' => {
                    let final_pos = i - *a;
                    let score = size - final_pos;
                    answer += score;
                }
                _ => (),
            }
        }
    }

    answer
}

fn main() {
    for input in INPUTS.iter() {
        println!("answer = {}", process(input));
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;
    let mut current = 0;

    for &c in data.as_bytes() {
        match c {
            b',' => {
                answer += current;
                current = 0;
            }

            b'\n' => (),
            c => {
                current += c as usize;
                current *= 17;
                current %= 256;
            }
        }
    }

    answer + current
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

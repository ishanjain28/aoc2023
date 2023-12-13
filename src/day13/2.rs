#![feature(iter_intersperse)]
#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;
    let mut horizontal_values: Vec<u32> = Vec::with_capacity(50);
    let mut vertical_values: Vec<u32> = Vec::with_capacity(50);

    for line in data.split("\n\n") {
        for line in line.lines() {
            let line_mask = line.bytes().fold(0u32, |a, x| a << 1 | (b'#' == x) as u32);

            horizontal_values.push(line_mask);
            vertical_values.resize(line.len(), 0);

            for (c, v) in line.bytes().zip(vertical_values.iter_mut()) {
                *v <<= 1;
                *v |= (c == b'#') as u32;
            }
        }

        let horiz_max = (0..horizontal_values.len())
            .map(|j| {
                let (a, b) = horizontal_values.split_at(j);
                let mut error_count = 0;

                for (a, b) in a.iter().rev().zip(b.iter()) {
                    error_count += (a ^ b).count_ones();

                    if error_count > 1 {
                        return 0;
                    }
                }

                if error_count == 1 {
                    j
                } else {
                    0
                }
            })
            .max()
            .unwrap_or_default();

        let vertical_max = (0..vertical_values.len())
            .map(|i| {
                let (a, b) = vertical_values.split_at(i);
                let mut error_count = 0;
                for (a, b) in a.iter().rev().zip(b.iter()) {
                    error_count += (a ^ b).count_ones();

                    if error_count > 1 {
                        return 0;
                    }
                }

                if error_count == 1 {
                    i
                } else {
                    0
                }
            })
            .max()
            .unwrap_or_default();

        answer += 100 * horiz_max;
        answer += vertical_max;

        horizontal_values.clear();
        vertical_values.clear();
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
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

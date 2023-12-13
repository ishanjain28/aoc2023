#![feature(iter_intersperse)]
#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;

    for line in data.split("\n\n") {
        let mut horiz_set = vec![];

        for line in line.lines() {
            let line_mask = line.chars().rev().enumerate().fold(0, |mut a, (i, x)| {
                if let '#' = x {
                    a |= 1 << (i)
                }
                a
            });

            horiz_set.push(line_mask);
        }

        let mut vert_set = vec![];

        for (i, line) in line.lines().rev().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if vert_set.len() <= j {
                    vert_set.push(0);
                }

                if c == '#' {
                    vert_set[j] |= 1 << i;
                }
            }
        }

        let horiz_max = (0..horiz_set.len())
            .rev()
            .map(|j| {
                let (a, b) = horiz_set.split_at(j);

                if a.iter().rev().zip(b.iter()).any(|(a, b)| a != b) {
                    return 0;
                }

                j
            })
            .max()
            .unwrap_or_default();

        let vertical_max = (0..vert_set.len())
            .rev()
            .map(|i| {
                let (a, b) = vert_set.split_at(i);

                if a.iter().rev().zip(b.iter()).any(|(a, b)| a != b) {
                    return 0;
                }

                i
            })
            .max()
            .unwrap_or_default();

        answer += 100 * horiz_max;
        answer += vertical_max;
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

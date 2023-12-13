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
                let mut error_count = 0;
                for (a, b) in a.iter().rev().zip(b.iter()) {
                    for x in 0..31 {
                        let a = a & (1 << x);
                        let b = b & (1 << x);

                        if a != b {
                            error_count += 1;
                        }
                    }

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

        let vertical_max = (0..vert_set.len())
            .rev()
            .map(|i| {
                let (a, b) = vert_set.split_at(i);
                let mut error_count = 0;
                for (a, b) in a.iter().rev().zip(b.iter()) {
                    for x in 0..31 {
                        let a = a & (1 << x);
                        let b = b & (1 << x);

                        if a != b {
                            error_count += 1;
                        }
                    }

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
    }

    answer
}

fn fmt(x: i32, length: usize) -> String {
    let mut out = String::new();

    for c in (0..length).rev() {
        let c = x & (1 << c);
        if c >= 1 {
            out.push('#');
        } else {
            out.push('.');
        }
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
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

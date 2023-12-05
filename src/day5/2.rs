#![feature(iter_array_chunks)]
#![feature(test)]

use std::ops::Range;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut lines = data.split("\n\n");

    let mut seeds: Vec<Range<i64>> = lines
        .next()
        .map(|x| {
            let v: Vec<i64> = x
                .split(' ')
                .skip(1)
                .map(|y| y.parse::<i64>().unwrap())
                .collect();

            v.chunks(2)
                .map(|x| (x[0]..x[0] + x[1]))
                .collect::<Vec<Range<i64>>>()
        })
        .unwrap();

    let maps: Vec<Vec<(Range<i64>, Range<i64>)>> = lines
        .map(|lines| {
            lines
                .lines()
                .skip(1)
                .map(|line| {
                    let y: Vec<i64> = line
                        .split_ascii_whitespace()
                        .map(|y| y.parse::<i64>().unwrap())
                        .collect();
                    let size = y[2];

                    (y[1]..y[1] + size, y[0]..y[0] + size)
                })
                .collect()
        })
        .collect();
    for map in maps {
        let mut out = vec![];

        for seed in seeds.drain(..) {
            //  split seed based on this rangemap
            let splitted = split(&map, seed);

            out.extend(splitted);
        }

        seeds.extend_from_slice(&out);
    }

    seeds.into_iter().map(|x| x.start).min().unwrap()
}

fn split(map: &[(Range<i64>, Range<i64>)], node: Range<i64>) -> Vec<Range<i64>> {
    let mut out = vec![];

    let mut stack = vec![node];

    while let Some(node) = stack.pop() {
        let mut found_match = false;

        for (src, dst) in map.iter() {
            let overlap = std::cmp::max(0, node.end.min(src.end) - node.start.max(src.start));

            if overlap == 0 {
                continue;
            } else if node.start > src.start && node.end < src.end {
                let offset = std::cmp::max(0, node.start - src.start);
                let dst = dst.start + offset..dst.start + offset + overlap;
                found_match = true;
                out.push(dst);
            } else if node.start < src.start && node.end > src.end {
                let r1_non = node.start..src.start;
                let r2_non = src.end..node.end;

                found_match = true;

                out.push(dst.clone());
                stack.push(r1_non);
                stack.push(r2_non);
            } else {
                // Partial overlap

                let offset = std::cmp::max(0, node.start - src.start);

                let dst_range = dst.start + offset..dst.start + offset + overlap;

                found_match = true;
                let new_range = if node.start < src.start {
                    node.start..src.start
                } else {
                    src.end..node.end
                };

                stack.push(new_range);
                out.push(dst_range);
            }
        }

        if !found_match {
            out.push(node);
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
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

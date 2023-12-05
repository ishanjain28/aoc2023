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
                .map(|x| x[0]..x[0] + x[1])
                .collect::<Vec<Range<i64>>>()
        })
        .unwrap();

    let maps = lines.map(|lines| {
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
            .collect::<Vec<(Range<i64>, Range<i64>)>>()
    });

    for map in maps {
        let mut out = Vec::with_capacity(seeds.len());

        for seed in seeds {
            //  split seed based on this rangemap
            let splitted = split(&map, seed);

            out.extend(splitted);
        }

        seeds = out
    }

    seeds.into_iter().map(|x| x.start).min().unwrap()
}

fn split(map: &[(Range<i64>, Range<i64>)], node: Range<i64>) -> Vec<Range<i64>> {
    let mut out = Vec::with_capacity(map.len());
    let mut stack = vec![node];

    while let Some(node) = stack.pop() {
        let mut found_match = false;

        for (src, dst) in map
            .iter()
            .skip_while(|(src, _)| node.end.min(src.end) - node.start.max(src.start) <= 0)
        {
            let overlap = std::cmp::max(0, node.end.min(src.end) - node.start.max(src.start));
            if overlap <= 0 {
                break;
            }

            let dst = if node.start == src.start && node.end == src.end {
                dst.clone()
            } else if node.start >= src.start && node.end <= src.end {
                // src engulfs node
                let offset = std::cmp::max(0, node.start - src.start);

                dst.start + offset..dst.start + offset + overlap
            } else if node.start < src.start && node.end > src.end {
                // node engulfs src
                let r1_non = node.start..src.start;
                let r2_non = src.end..node.end;

                stack.extend([r1_non, r2_non]);

                dst.clone()
            } else {
                // Partial overlap

                let offset = std::cmp::max(0, node.start - src.start);
                let dst_range = dst.start + offset..dst.start + offset + overlap;

                let new_range = if node.start < src.start {
                    node.start..src.start
                } else {
                    src.end..node.end
                };

                stack.push(new_range);
                dst_range
            };

            found_match = true;
            out.push(dst);
        }

        if !found_match {
            out.push(node);
        }
    }

    out
}

fn main() {
    for input in INPUTS.iter() {
        println!("answer = {}", process(input));
    }
}

#[bench]
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

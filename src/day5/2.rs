#![feature(const_trait_impl)]
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

    let mut out = Vec::with_capacity(seeds.len());
    for map in maps {
        for (src, dst) in map.iter() {
            let mut i = 0;
            while i < seeds.len() {
                let seed = &seeds[i];

                let [lno, overlap, rno] = intersect(src, &seed);

                if !overlap.is_empty() {
                    out.push(
                        overlap.start - src.start + dst.start..overlap.end - src.start + dst.start,
                    );
                    seeds.swap_remove(i);

                    if !lno.is_empty() {
                        seeds.push(lno);
                    }
                    if !rno.is_empty() {
                        seeds.push(rno);
                    }
                } else {
                    i += 1;
                }
            }
        }

        seeds.append(&mut out);
    }

    seeds.into_iter().map(|x| x.start).min().unwrap()
}

#[inline]
fn intersect(node: &Range<i64>, src: &Range<i64>) -> [Range<i64>; 3] {
    let overlap = src.start.max(node.start)..src.end.min(node.end);
    let left_nonoverlap = src.start..overlap.start;
    let right_nonoverlap = overlap.end..src.end;

    [left_nonoverlap, overlap, right_nonoverlap]
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

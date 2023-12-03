#![feature(test)]

use std::collections::{HashMap, HashSet};
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

fn process(data: &[u8]) -> u64 {
    let mut total = 0;

    let grid: Vec<&[u8]> = data
        .split(|&x| x == b'\n')
        .filter(|x| !x.is_empty())
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut map = HashMap::with_capacity(200);

    for i in 0..m {
        for j in 0..n {
            let c = grid[i][j];

            if c != b'*' {
                continue;
            }

            for (p, q) in DIRS.iter() {
                let x = i as i32 + p;
                let y = j as i32 + q;

                if x < 0
                    || y < 0
                    || x >= m as i32
                    || y >= n as i32
                    || !grid[x as usize][y as usize].is_ascii_digit()
                {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;

                let mut sy = y;
                let mut ey = y;

                while sy > 0 && grid[x][sy - 1].is_ascii_digit() {
                    sy -= 1;
                }
                while ey < n && grid[x][ey].is_ascii_digit() {
                    ey += 1;
                }

                let num = String::from_utf8_lossy(&grid[x][sy..ey])
                    .parse::<u64>()
                    .unwrap();

                map.entry((i, j))
                    .or_insert_with(HashSet::new)
                    .insert((num, x, sy, ey));
            }
        }
    }

    for v in map.into_values() {
        if v.len() < 2 {
            continue;
        }

        total += v.into_iter().fold(1, |a, (b, _, _, _)| a * b);
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

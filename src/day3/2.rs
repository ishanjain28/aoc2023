#![feature(test)]

use std::collections::HashSet;
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

    let mut set = HashSet::with_capacity(2);
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != b'*' {
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

                // only insert valid parts in this
                if set.len() < 2 {
                    set.insert((parse(&grid[x][sy..ey]), sy, ey, x));
                }
            }

            if set.len() == 2 {
                total += set.drain().fold(1, |a, (x, _, _, _)| a * x);
            } else {
                set.clear();
            }
        }
    }

    total
}

fn parse(b: &[u8]) -> u64 {
    let mut output = 0;

    for (i, v) in b.iter().enumerate() {
        output += 10u64.pow(b.len() as u32 - i as u32 - 1) * ((v - b'0') as u64);
    }

    output
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

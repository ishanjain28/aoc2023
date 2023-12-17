#![feature(test)]

use std::{cmp::Reverse, collections::BinaryHeap};
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

fn process(data: &str) -> i64 {
    let size = data.lines().count() as i64;
    let data = data.as_bytes();

    let mut visited: BitSet<80000> = BitSet::new();

    use Direction::*;

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0i64, 0i64, 1, East));

    while let Some((Reverse(cost), x, y, in_same_dir, dir)) = q.pop() {
        if x == size - 1 && y == size - 1 {
            return cost;
        }

        if visited.get(dir as i64 * size * size * size + in_same_dir * size * size + y * size + x) {
            continue;
        } else {
            visited.set(dir as i64 * size * size * size + in_same_dir * size * size + y * size + x);
        }

        let coords = [(0, -1, West), (1, 0, South), (-1, 0, North), (0, 1, East)];
        for &(r, s, ndir) in coords.iter() {
            if in_same_dir >= 3 && ndir == dir
                || (dir == West && ndir == East)
                || (dir == East && ndir == West)
                || (dir == North && ndir == South)
                || (dir == South && ndir == North)
            {
                continue;
            }

            let x = x + r;
            let y = y + s;
            if x < 0 || y < 0 || x >= size || y >= size {
                continue;
            }

            let new_cost = cost + (data[(x * size + y + x) as usize] - b'0') as i64;

            q.push((
                Reverse(new_cost),
                x,
                y,
                if dir == ndir { in_same_dir + 1 } else { 1 },
                ndir,
            ));
        }
    }

    std::i64::MAX
}

struct BitSet<const N: usize> {
    bits: [u128; N],
}

impl<const N: usize> BitSet<N> {
    const fn new() -> Self {
        Self { bits: [0; N] }
    }

    #[inline]
    fn set(&mut self, loc: i64) {
        let idx = loc / 128;
        let b_idx = loc & 127;

        self.bits[idx as usize] |= 1 << b_idx;
    }

    #[inline]
    const fn get(&self, loc: i64) -> bool {
        let idx = loc / 128;
        let b_idx = loc & 127;

        (self.bits[idx as usize] & (1 << b_idx)) > 0
    }
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

#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;

    let grid: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();

    let m = grid.len() as i64;
    let n = grid[0].len() as i64;

    let mut stack = Vec::new();

    stack.push((0, 0, 1, BitSet::<156>::new()));

    while let Some((distance, sx, sy, mut visited)) = stack.pop() {
        if sx == m - 1 && sy == n - 2 {
            answer = std::cmp::max(answer, distance);
            continue;
        }

        if unsafe { grid.get_unchecked(sx as usize).get_unchecked(sy as usize) == &'#' } {
            continue;
        }
        if visited.get(sx * m + sy) {
            continue;
        }
        visited.set(sx * m + sy);

        if sy + 1 < n {
            stack.push((distance + 1, sx, sy + 1, visited.clone()));
        }
        if sy > 0 {
            stack.push((distance + 1, sx, sy - 1, visited.clone()));
        }
        if sx + 1 < n {
            stack.push((distance + 1, sx + 1, sy, visited.clone()));
        }
        if sx > 0 {
            stack.push((distance + 1, sx - 1, sy, visited));
        }
    }

    answer
}

fn main() {
    for input in INPUTS.iter() {
        println!("answer = {}", process(input));
    }
}

#[derive(Ord, PartialEq, PartialOrd, Clone, Debug, Eq)]
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
#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

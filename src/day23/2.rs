#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn process(data: &str) -> i64 {
    let mut answer = 0;

    let size = data.lines().count() as i64;

    let data = data.as_bytes();

    use Direction::*;
    let mut stack = Vec::new();
    stack.push((0, 0, 1, BitSet::<156>::new(), South));

    while let Some((distance, sx, sy, mut visited, direction)) = stack.pop() {
        if sx == size - 1 && sy == size - 2 {
            answer = std::cmp::max(answer, distance);
            continue;
        }

        if unsafe { *data.get_unchecked((sx * size + sy + sx) as usize) == b'#' } {
            continue;
        }
        if visited.get(sx * size + sy) {
            continue;
        }
        visited.set(sx * size + sy);

        if sy + 1 < size && direction != West {
            stack.push((distance + 1, sx, sy + 1, visited.clone(), East));
        }
        if sy > 0 && direction != East {
            stack.push((distance + 1, sx, sy - 1, visited.clone(), West));
        }
        if sx + 1 < size && direction != North {
            stack.push((distance + 1, sx + 1, sy, visited.clone(), South));
        }
        if sx > 0 && direction != South {
            stack.push((distance + 1, sx - 1, sy, visited, North));
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

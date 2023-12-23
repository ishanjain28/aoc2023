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

impl Direction {
    #[inline]
    const fn coords(&self) -> [([i64; 2], Self); 3] {
        use Direction::*;

        match self {
            Direction::East => [([0, 1], East), ([1, 0], South), ([-1, 0], North)],
            Direction::West => [([0, -1], West), ([1, 0], South), ([-1, 0], North)],
            Direction::North => [([0, 1], East), ([0, -1], West), ([-1, 0], North)],
            Direction::South => [([0, 1], East), ([0, -1], West), ([1, 0], South)],
        }
    }
}

fn process(data: &str) -> i64 {
    let mut answer = 0;
    let size = data.lines().count() as i64;
    let data = data.as_bytes();

    let mut stack = Vec::new();
    use Direction::*;
    stack.push((0, 0, 1, BitSet::<156>::new(), South));

    while let Some((distance, sx, sy, mut visited, direction)) = stack.pop() {
        if sx == size - 1 && sy == size - 2 {
            answer = std::cmp::max(answer, distance);
            continue;
        }
        if visited.get(sx * size + sy) {
            continue;
        }
        visited.set(sx * size + sy);

        let c = unsafe { *data.get_unchecked((sx * size + sy + sx) as usize) };
        let sticky = match c {
            b'>' => Some([0, 1]),
            b'<' => Some([0, -1]),
            b'v' => Some([1, 0]),
            b'^' => Some([-1, 0]),
            _ => None,
        };

        if let Some(d) = sticky {
            let mut x = sx;
            let mut y = sy;
            let mut dis = 0;
            loop {
                let p = x + d[0];
                let q = y + d[1];

                if p < 0
                    || q < 0
                    || p >= size
                    || q >= size
                    || unsafe { *data.get_unchecked((p * size + q + p) as usize) } != b'.'
                {
                    break;
                }

                x = p;
                y = q;
                dis += 1;
            }

            stack.push((distance + dis, x, y, visited, direction));
        } else {
            for (dir, direction) in direction.coords().iter() {
                let a = dir[0] + sx;
                let b = dir[1] + sy;

                if a < 0 || b < 0 || a >= size || b >= size {
                    continue;
                }

                if unsafe { *data.get_unchecked((a * size + b + a) as usize) == b'#' } {
                    continue;
                }

                stack.push((distance + 1, a, b, visited.clone(), *direction));
            }
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

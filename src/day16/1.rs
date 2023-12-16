#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

fn process(data: &[u8]) -> usize {
    let size = data.iter().filter(|&&x| x == b'\n').count();
    use Direction::*;

    solve(data, size as i32, (0, 0, East))
}

fn solve(grid: &[u8], size: i32, (sx, sy, dir): (i32, i32, Direction)) -> usize {
    use Direction::*;

    let mut visited: BitSet<100> = BitSet::new();
    let mut seen: BitSet<400> = BitSet::new();

    let mut stack = vec![];

    stack.push((sx, sy, dir));

    while let Some((sx, sy, dir)) = stack.pop() {
        if sx < 0 || sy < 0 || sx >= size || sy >= size {
            continue;
        }

        if seen.get(size * size * dir as i32 + size * sx + sy) {
            continue;
        } else {
            seen.set(size * size * dir as i32 + size * sx + sy);
        }

        visited.set(sx * size + sy);
        let c = grid[(sx * size + sy + sx) as usize];

        let mut x = sx;
        let mut y = sy;

        match (c, dir) {
            (b'\\', East) => stack.push((x + 1, y, South)),
            (b'\\', North) => stack.push((x, y - 1, West)),
            (b'\\', West) => stack.push((x - 1, y, North)),
            (b'\\', South) => stack.push((x, y + 1, East)),
            (b'/', East) => stack.push((x - 1, y, North)),
            (b'/', West) => stack.push((x + 1, y, South)),
            (b'/', South) => stack.push((x, y - 1, West)),
            (b'/', North) => stack.push((x, y + 1, East)),
            (b'|', North) => stack.push((x - 1, y, dir)),
            (b'|', South) => stack.push((x + 1, y, dir)),
            (b'|', _) => {
                stack.push((x - 1, y, North));
                stack.push((x + 1, y, South));
            }
            (b'-', East) => stack.push((x, y + 1, dir)),
            (b'-', West) => stack.push((x, y - 1, dir)),
            (b'-', _) => {
                stack.push((x, y + 1, East));
                stack.push((x, y - 1, West));
            }
            (b'.', _) => {
                let (r, s) = match dir {
                    North => (-1, 0),
                    South => (1, 0),
                    East => (0, 1),
                    West => (0, -1),
                };

                loop {
                    visited.set(x * size + y);
                    x += r;
                    y += s;
                    if x < 0 || y < 0 || x >= size || y >= size {
                        break;
                    }
                    let c = grid[(x * size + y + x) as usize];
                    if c != b'.' {
                        break;
                    }
                }
                if !(x < 0 || y < 0 || x >= size || y >= size) {
                    stack.push((x, y, dir));
                }
            }

            _ => unreachable!(),
        }
    }

    visited.count()
}

struct BitSet<const N: usize> {
    bits: [u128; N],
}

impl<const N: usize> BitSet<N> {
    const fn new() -> Self {
        Self { bits: [0; N] }
    }

    #[inline]
    fn set(&mut self, loc: i32) {
        let idx = loc / 128;
        let b_idx = loc & 127;

        self.bits[idx as usize] |= 1 << b_idx;
    }

    #[inline]
    const fn get(&self, loc: i32) -> bool {
        let idx = loc / 128;
        let b_idx = loc & 127;

        (self.bits[idx as usize] & (1 << b_idx)) > 0
    }

    const fn count(&self) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < self.bits.len() * 128 {
            count += self.get(i as i32) as usize;
            i += 1;
        }

        count
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

#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let mut answer = 0;

    let grid: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut stack = Vec::new();
    stack.push((0, 0, 1, BitSet::<156>::new()));

    while let Some((distance, sx, sy, mut visited)) = stack.pop() {
        if sx == m as i64 - 1 && sy == n as i64 - 2 {
            answer = std::cmp::max(answer, distance);
            continue;
        }
        if visited.get(sx * m as i64 + sy) {
            continue;
        }
        visited.set(sx * m as i64 + sy);

        let c = grid[sx as usize][sy as usize];
        let sticky = match c {
            '>' => Some([0, 1]),
            '<' => Some([0, -1]),
            'v' => Some([1, 0]),
            '^' => Some([-1, 0]),
            _ => None,
        };
        const DIRS: [[i64; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        if let Some(d) = sticky {
            let mut x = sx;
            let mut y = sy;
            let mut dis = 0;
            loop {
                let p = x + d[0];
                let q = y + d[1];

                if p < 0
                    || q < 0
                    || p >= m as i64
                    || q >= n as i64
                    || grid[p as usize][q as usize] != '.'
                {
                    break;
                }

                x = p;
                y = q;
                dis += 1;
            }

            stack.push((distance + dis, x, y, visited));
        } else {
            for dir in DIRS.iter() {
                let a = dir[0] + sx;
                let b = dir[1] + sy;

                if a < 0 || b < 0 || a >= m as i64 || b >= n as i64 {
                    continue;
                }

                if unsafe { grid.get_unchecked(a as usize).get_unchecked(b as usize) == &'#' } {
                    continue;
                }

                stack.push((distance + 1, a, b, visited.clone()));
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

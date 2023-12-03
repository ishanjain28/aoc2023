#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

const DIRS: [(i32, i32); 6] = [(0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)];
const TOP_DOWN: [(i32, i32); 2] = [(1, 0), (-1, 0)];

fn process(data: &[u8]) -> u64 {
    let grid: Vec<&[u8]> = data
        .split(|&x| x == b'\n')
        .filter(|x| !x.is_empty())
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut total = 0;

    for i in 0..m {
        let mut j = 0;
        while j < n {
            if !grid[i][j].is_ascii_digit() {
                j += 1;
                continue;
            }

            let mut ey = j;
            while ey < n && grid[i][ey].is_ascii_digit() {
                ey += 1;
            }

            'outer: for p in j..ey {
                for (a, _) in TOP_DOWN.iter() {
                    let x = i as i32 + a;
                    if x < 0 || x >= m as i32 {
                        continue;
                    }

                    let c = grid[x as usize][p];
                    if c != b'.' && c.is_ascii_punctuation() {
                        total += parse(&grid[i][j..ey]);
                        break 'outer;
                    }
                }

                if p == j || p == ey - 1 {
                    for (a, b) in DIRS.iter() {
                        let x = i as i32 + a;
                        let y = p as i32 + b;
                        if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                            continue;
                        }

                        let c = grid[x as usize][y as usize];
                        if c != b'.' && c.is_ascii_punctuation() {
                            total += parse(&grid[i][j..ey]);
                            break 'outer;
                        }
                    }
                }
            }

            j = ey;
        }
    }

    total
}

#[inline]
fn parse(b: &[u8]) -> u64 {
    let mut output = 0;
    let mut pow = 1;

    for v in b.iter().rev() {
        output += pow * ((v - b'0') as u64);
        pow *= 10;
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

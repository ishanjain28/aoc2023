#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;

    let mut grid: Vec<Vec<u8>> = data.lines().map(|x| x.bytes().collect()).collect();

    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            let c = grid[i][j];

            if c != b'O' {
                continue;
            }

            // move the rock north
            let mut start = i;
            while start > 0 && grid[start - 1][j] == b'.' {
                start -= 1;
            }

            grid[i][j] = b'.';
            grid[start][j] = b'O';
        }
    }

    for (i, line) in grid.iter().enumerate() {
        for &c in line {
            if c == b'O' {
                answer += m - i;
            }
        }
    }

    answer
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

#![feature(iter_intersperse)]
#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    const N: i32 = 64;

    let mut grid: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();

    let m = grid.len();
    let n = grid[0].len();

    let mut sx = 0;
    let mut sy = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'S' {
                sx = i;
                sy = j;
                break;
            }
        }
    }

    grid[sx][sy] = '.';

    let mut visited = vec![vec![-1; n]; m];

    for i in 0..m {
        for j in 0..m {
            if grid[i][j] == '#' {
                visited[i][j] = std::i32::MIN;
            } else {
                visited[i][j] = -1
            }
        }
    }
    visited[sx][sy] = 0;

    for step in 1..=64 {
        let mut next_visited = visited.clone();

        for i in 0..m {
            for j in 0..n {
                if visited[i][j] == std::i32::MIN {
                    continue;
                }
                if step - visited[i][j] > 1 {
                    continue;
                }

                for (x, y) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)].iter() {
                    let p = x + i as i32;
                    let q = y + j as i32;

                    if p < 0 || q < 0 || p >= m as i32 || q >= n as i32 {
                        continue;
                    }

                    if grid[p as usize][q as usize] == '#' {
                        continue;
                    }
                    if step - visited[p as usize][q as usize] == 1 {
                        continue;
                    }

                    next_visited[p as usize][q as usize] = step;
                }
            }
        }

        visited = next_visited;
    }

    visited
        .into_iter()
        .fold(0, |a, x| a + x.into_iter().filter(|&x| x == N).count()) as i64
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

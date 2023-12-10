#![feature(test)]

use std::collections::VecDeque;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = std::usize::MAX;

    let grid: Vec<Vec<char>> = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    let (mut s_x, mut s_y) = (0, 0);

    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'S' {
                s_x = i;
                s_y = j;
            }
        }
    }

    let mut map = vec![vec!['.'; n]; m];

    for mut c in ['F'] {
        let mut sx = s_x;
        let mut sy = s_y;
        let (mut px, mut py) = (s_x, s_y);
        let mut length = 0;

        map[sx][sy] = c;

        while c != 'S' {
            println!(
                "({}, {}) => ({}, {}) c = {} {}",
                px, py, sx, sy, c, grid[sx][sy]
            );

            match c {
                '|' => {
                    if sx > 0 && !(sx - 1 == px && sy == py) {
                        px = sx;
                        py = sy;
                        sx = sx - 1;
                        length += 1;
                        c = grid[sx][sy];
                    } else if sx < m - 1 && !(sx + 1 == px && sy == py) {
                        px = sx;
                        py = sy;
                        sx = sx + 1;
                        length += 1;
                        c = grid[sx][sy];
                    }
                }
                'F' => {
                    if sy < n - 1 && !(sx == px && sy + 1 == py) {
                        px = sx;
                        py = sy;
                        sy = sy + 1;
                        length += 1;
                        c = grid[sx][sy];
                    } else if sx < m - 1 && !(sx + 1 == px && px == py) {
                        px = sx;
                        py = sy;
                        sx = sx + 1;
                        length += 1;
                        c = grid[sx][sy];
                    }
                }

                '-' => {
                    if sy > 0 && !(sx == px && sy - 1 == py) {
                        px = sx;
                        py = sy;
                        sy = sy - 1;
                        length += 1;
                        c = grid[sx][sy];
                    } else if sy < n - 1 && !(sx == px && sy + 1 == py) {
                        px = sx;
                        py = sy;
                        sy = sy + 1;
                        length += 1;
                        c = grid[sx][sy];
                    }
                }

                '7' => {
                    if sy > 0 && !(sx == px && sy - 1 == py) {
                        px = sx;
                        py = sy;
                        sy = sy - 1;
                        length += 1;
                        c = grid[sx][sy];
                    } else if sx < m - 1 && !(sx + 1 == px && sy == py) {
                        px = sx;
                        py = sy;
                        sx = sx + 1;
                        length += 1;
                        c = grid[sx][sy];
                    }
                }

                'J' => {
                    if sy > 0 && !(sx == px && sy - 1 == py) {
                        px = sx;
                        py = sy;
                        sy = sy - 1;
                        length += 1;
                        c = grid[sx][sy];
                    } else if sx > 0 && !(sx - 1 == px && sy == py) {
                        px = sx;
                        py = sy;
                        sx = sx - 1;
                        length += 1;
                        c = grid[sx][sy];
                    }
                }

                'L' => {
                    if sx > 0 && !(sx - 1 == px && sy == py) {
                        px = sx;
                        py = sy;
                        sx = sx - 1;
                        length += 1;
                        c = grid[sx][sy];
                    } else if sy < n - 1 && !(sx == px && sy + 1 == py) {
                        px = sx;
                        py = sy;
                        sy = sy + 1;
                        length += 1;
                        c = grid[sx][sy];
                    }
                }
                '.' => break,

                _ => unreachable!(),
            }
        }

        answer = std::cmp::min(answer, length / 2);
    }

    for line in map {
        println!("{:?}", line);
    }

    answer
}

fn dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    path: &mut Vec<char>,
    sx: usize,
    sy: usize,
    c: char,
    distance: usize,
) {
    if visited[sx][sy] {
        //    return;
    } else {
        visited[sx][sy] = true;
    }

    if c == 'S' {
        return;
    }

    path.push(c);

    println!("({},{}) = {} {}", sx, sy, c, distance);

    let m = grid.len();
    let n = grid[0].len();

    match c {
        '|' => {
            if sx > 0 {
                let c = grid[sx - 1][sy];
                dfs(grid, visited, path, sx - 1, sy, c, distance + 1);
            }
            if sx < m - 1 {
                let c = grid[sx + 1][sy];
                dfs(grid, visited, path, sx + 1, sy, c, distance + 1);
            }
        }

        '-' => {
            if sy > 0 {
                let c = grid[sx][sy - 1];
                dfs(grid, visited, path, sx, sy - 1, c, distance + 1);
            }
            if sy < n - 1 {
                let c = grid[sx][sy + 1];
                dfs(grid, visited, path, sx, sy + 1, c, distance + 1);
            }
        }

        'L' => {
            if sx > 0 {
                let c = grid[sx - 1][sy];
                dfs(grid, visited, path, sx - 1, sy, c, distance + 1);
            }
            if sy < n - 1 {
                let c = grid[sx][sy + 1];
                dfs(grid, visited, path, sx, sy + 1, c, distance + 1);
            }
        }

        'J' => {
            if sy > 0 {
                let c = grid[sx][sy - 1];
                dfs(grid, visited, path, sx, sy - 1, c, distance + 1);
            }
            if sx > 0 {
                let c = grid[sx - 1][sy];
                dfs(grid, visited, path, sx - 1, sy, c, distance + 1);
            }
        }

        '7' => {
            if sy > 0 {
                let c = grid[sx][sy - 1];
                dfs(grid, visited, path, sx, sy - 1, c, distance + 1);
            }
            if sx < m - 1 {
                let c = grid[sx + 1][sy];
                dfs(grid, visited, path, sx + 1, sy, c, distance + 1);
            }
        }

        'F' => {
            if sy < n - 1 {
                let c = grid[sx][sy + 1];
                dfs(grid, visited, path, sx, sy + 1, c, distance + 1);
            }
            if sx < m - 1 {
                let c = grid[sx + 1][sy];
                dfs(grid, visited, path, sx + 1, sy, c, distance + 1);
            }
        }

        '.' => (),

        _ => unreachable!(),
    }

    path.pop();
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
        break;
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

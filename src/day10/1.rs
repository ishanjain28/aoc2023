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

    for mut c in ['F', 'L', '|', '-', 'J', '7'] {
        let mut sx = s_x;
        let mut sy = s_y;
        let (mut px, mut py) = (s_x, s_y);
        let mut length = 0;

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

    return answer;

    //  let mut stack = vec![];
    //  //    let mut q = VecDeque::new();
    //  stack.push((sx, sy, 'F', sx, sy, 0));
    //  stack.push((sx, sy, 'L', sx, sy, 0));

    let mut min_dist = vec![vec![std::usize::MAX; n]; m];

    //    while let Some((sx, sy, c, px, py, distance)) = stack.pop() {
    //        min_dist[sx][sy] = std::cmp::min(min_dist[sx][sy], distance);
    //        match c {
    //            '|' => {
    //                if sx > 0 && !(sx - 1 == px && sy == py) {
    //                    let c = grid[sx - 1][sy];
    //                    stack.push((sx - 1, sy, c, sx, sy, distance + 1));
    //                }
    //                if sx < m - 1 && !(sx + 1 == px && sy == py) {
    //                    let c = grid[sx + 1][sy];
    //                    stack.push((sx + 1, sy, c, sx, sy, distance + 1));
    //                }
    //            }
    //
    //            '-' => {
    //                if sy > 0 && !(sx == px && sy - 1 == py) {
    //                    let c = grid[sx][sy - 1];
    //                    stack.push((sx, sy - 1, c, sx, sy, distance + 1));
    //                }
    //                if sy < n - 1 && !(sx == px && sy + 1 == py) {
    //                    let c = grid[sx][sy + 1];
    //                    stack.push((sx, sy + 1, c, sx, sy, distance + 1));
    //                }
    //            }
    //
    //            'L' => {
    //                if sx > 0 && !(sx - 1 == px && sy == py) {
    //                    let c = grid[sx - 1][sy];
    //                    stack.push((sx - 1, sy, c, sx, sy, distance + 1));
    //                }
    //                if sy < n - 1 && !(sx == px && sy + 1 == py) {
    //                    let c = grid[sx][sy + 1];
    //                    stack.push((sx, sy + 1, c, sx, sy, distance + 1));
    //                }
    //            }
    //
    //            'J' => {
    //                if sy > 0 && !(sx == px && sy - 1 == py) {
    //                    let c = grid[sx][sy - 1];
    //                    stack.push((sx, sy - 1, c, sx, sy, distance + 1));
    //                }
    //                if sx > 0 && !(sx - 1 == px && sy == py) {
    //                    let c = grid[sx - 1][sy];
    //                    stack.push((sx - 1, sy, c, sx, sy, distance + 1));
    //                }
    //            }
    //
    //            '7' => {
    //                if sy > 0 && !(sx == px && sy - 1 == py) {
    //                    let c = grid[sx][sy - 1];
    //                    stack.push((sx, sy - 1, c, sx, sy, distance + 1));
    //                }
    //                if sx < m - 1 && !(sx + 1 == px && sy == py) {
    //                    let c = grid[sx + 1][sy];
    //                    stack.push((sx + 1, sy, c, sx, sy, distance + 1));
    //                }
    //            }
    //
    //            'F' => {
    //                if sy < n - 1 && !(sx == px && sy + 1 == py) {
    //                    let c = grid[sx][sy + 1];
    //                    stack.push((sx, sy + 1, c, sx, sy, distance + 1));
    //                }
    //                if sx < m - 1 && !(sx + 1 == px && px == py) {
    //                    let c = grid[sx + 1][sy];
    //                    stack.push((sx + 1, sy, c, sx, sy, distance + 1));
    //                }
    //            }
    //
    //            '.' => (),
    //
    //            'S' => (),
    //
    //            v => {
    //                unreachable!()
    //            }
    //        }
    //    }
    //
    println!();

    for line in min_dist {
        println!("{:?}", line);

        for c in line {
            if c == std::usize::MAX {
                continue;
            }

            answer = std::cmp::max(answer, c);
        }
    }
    println!();

    //    for &(sx, sy, c) in [(sx, sy, 'F')].iter() {
    //        let mut visited = vec![vec![false; n]; m];
    //
    //        // dfs(&grid, &mut visited, &mut path, sx, sy, c, 0);
    //    }

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
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

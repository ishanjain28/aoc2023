#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;
    let mut grid: Vec<Vec<char>> = data
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

    'outer: for mut c in ['|', '-', 'L', 'J', '7', 'F'] {
        let mut sx = s_x;
        let mut sy = s_y;
        let (mut px, mut py) = (s_x, s_y);
        let mut start = true;

        grid[sx][sy] = c;

        let mut tmp_map = map.clone();

        while start || !(sx == s_x && sy == s_y) {
            start = false;

            tmp_map[sx][sy] = c;

            match c {
                '|' => {
                    if sx > 0
                        && !(sx - 1 == px && sy == py)
                        && ['7', 'F', '|', 'S'].contains(&grid[sx - 1][sy])
                    {
                        px = sx;
                        py = sy;
                        sx -= 1;
                        c = grid[sx][sy];
                    } else if sx < m - 1
                        && !(sx + 1 == px && sy == py)
                        && ['L', 'J', '|', 'S'].contains(&grid[sx + 1][sy])
                    {
                        px = sx;
                        py = sy;
                        sx += 1;
                        c = grid[sx][sy];
                    } else {
                        continue 'outer;
                    }
                }
                'F' => {
                    if sy < n - 1
                        && !(sx == px && sy + 1 == py)
                        && ['-', 'J', '7', 'S'].contains(&grid[sx][sy + 1])
                    {
                        px = sx;
                        py = sy;
                        sy += 1;
                        c = grid[sx][sy];
                    } else if sx < m - 1
                        && !(sx + 1 == px && px == py)
                        && ['L', 'J', '|', 'S'].contains(&grid[sx + 1][sy])
                    {
                        px = sx;
                        py = sy;
                        sx += 1;
                        c = grid[sx][sy];
                    } else {
                        continue 'outer;
                    }
                }

                '-' => {
                    if sy > 0
                        && !(sx == px && sy - 1 == py)
                        && ['L', 'F', '-', 'S'].contains(&grid[sx][sy - 1])
                    {
                        px = sx;
                        py = sy;
                        sy -= 1;
                        c = grid[sx][sy];
                    } else if sy < n - 1
                        && !(sx == px && sy + 1 == py)
                        && ['J', '7', '-', 'S'].contains(&grid[sx][sy + 1])
                    {
                        px = sx;
                        py = sy;
                        sy += 1;
                        c = grid[sx][sy];
                    } else {
                        continue 'outer;
                    }
                }

                '7' => {
                    if sy > 0
                        && !(sx == px && sy - 1 == py)
                        && ['L', 'F', '-', 'S'].contains(&grid[sx][sy - 1])
                    {
                        px = sx;
                        py = sy;
                        sy -= 1;
                        c = grid[sx][sy];
                    } else if sx < m - 1
                        && !(sx + 1 == px && sy == py)
                        && ['L', 'J', '|', 'S'].contains(&grid[sx + 1][sy])
                    {
                        px = sx;
                        py = sy;
                        sx += 1;
                        c = grid[sx][sy];
                    } else {
                        continue 'outer;
                    }
                }

                'J' => {
                    if sy > 0
                        && !(sx == px && sy - 1 == py)
                        && ['L', 'F', '-', 'S'].contains(&grid[sx][sy - 1])
                    {
                        px = sx;
                        py = sy;
                        sy -= 1;
                        c = grid[sx][sy];
                    } else if sx > 0
                        && !(sx - 1 == px && sy == py)
                        && ['7', 'F', '|', 'S'].contains(&grid[sx - 1][sy])
                    {
                        px = sx;
                        py = sy;
                        sx -= 1;
                        c = grid[sx][sy];
                    } else {
                        continue 'outer;
                    }
                }

                'L' => {
                    if sx > 0
                        && !(sx - 1 == px && sy == py)
                        && ['7', 'F', '|', 'S'].contains(&grid[sx - 1][sy])
                    {
                        px = sx;
                        py = sy;
                        sx -= 1;
                        c = grid[sx][sy];
                    } else if sy < n - 1
                        && !(sx == px && sy + 1 == py)
                        && ['J', '7', '-', 'S'].contains(&grid[sx][sy + 1])
                    {
                        px = sx;
                        py = sy;
                        sy += 1;
                        c = grid[sx][sy];
                    } else {
                        continue 'outer;
                    }
                }
                '.' => continue 'outer,

                _ => unreachable!(),
            }
        }

        match (
            sx as i32 - px as i32,
            sy as i32 - py as i32,
            grid[px][py],
            grid[sx][sy],
        ) {
            (-1, 0, 'J', 'F') => (),
            (-1, 0, '|', '|') => {
                if sx == 0 || sx == m - 1 {
                    continue;
                }
            }
            (-1, 0, '|', '7') => (),
            (-1, 0, '|', 'F') => (),
            (0, -1, 'J', 'L') => (),

            _ => unreachable!(),
        };

        if sx == s_x && sy == s_y {
            map = tmp_map;
            break;
        }
    }

    let mut nmap = vec![vec!['.'; n * 2]; m * 2];

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            nmap[i * 2][j * 2] = *c;
        }
    }

    for line in nmap.iter_mut() {
        for j in 0..2 * n - 2 {
            if ['F', 'L', '-'].contains(&line[j]) && ['J', '7', '-'].contains(&line[j + 2]) {
                line[j + 1] = '-';
            }
        }
    }

    for j in 0..n * 2 {
        for i in 0..m * 2 - 2 {
            if ['F', '7', '|'].contains(&nmap[i][j]) && ['L', 'J', '|'].contains(&nmap[i + 2][j]) {
                nmap[i + 1][j] = '|';
            }
        }
    }

    for j in 0..2 * n {
        if nmap[0][j] == '.' {
            flood_fill(&mut nmap, 0, j, 'X');
        }
    }

    let mut out = vec![vec!['.'; n]; m];

    for (i, line) in nmap.iter().enumerate() {
        if i % 2 != 0 {
            continue;
        }

        for (j, c) in line.iter().enumerate() {
            if j % 2 != 0 {
                continue;
            }

            out[i / 2][j / 2] = *c;
        }
    }

    for line in out.into_iter() {
        for c in line.into_iter() {
            if c == '.' {
                answer += 1;
            }
        }
    }

    answer
}

fn flood_fill(map: &mut Vec<Vec<char>>, i: usize, j: usize, fill_with: char) {
    let m = map.len();
    let n = map[0].len();
    const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [-1, 0], [1, 0]];

    let mut stack = vec![];
    stack.push((i, j));

    while let Some((sx, sy)) = stack.pop() {
        if map[sx][sy] != '.' {
            continue;
        }

        map[sx][sy] = fill_with;

        for dir in DIRS.iter() {
            let x = sx as i32 + dir[0];
            let y = sy as i32 + dir[1];

            if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            stack.push((x, y));
        }
    }
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

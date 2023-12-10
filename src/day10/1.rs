#![feature(test)]

extern crate test;

const INPUTS: [&str; 4] = [
    ".....
.S-7.
.|.|.
.L-J.
.....",
    "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    include_str!("./sample.txt"),
    include_str!("./input.txt"),
];

#[derive(Debug)]
enum Direction {
    Unknown,
    North,
    South,
    East,
    West,
}

#[inline]
const fn next_direction(a: char, dir: &Direction) -> (Direction, [char; 3]) {
    use Direction::*;
    match (a, dir) {
        ('|', Unknown | North) => (North, ['|', '7', 'F']),
        ('|', South) => (South, ['|', 'L', 'J']),

        ('-', Unknown | West) => (West, ['-', 'L', 'F']),
        ('-', East) => (East, ['-', 'J', '7']),

        ('L', Unknown | West) => (North, ['|', '7', 'F']),
        ('L', South) => (East, ['-', 'J', '7']),

        ('J', Unknown | South) => (West, ['-', 'L', 'F']),
        ('J', East) => (North, ['|', 'F', '7']),

        ('7', Unknown | East) => (South, ['|', 'L', 'J']),
        ('7', North) => (West, ['-', 'L', 'F']),

        ('F', Unknown | North) => (East, ['-', '7', 'J']),
        ('F', West) => (South, ['|', 'L', 'J']),

        _ => unreachable!(),
    }
}

fn process(data: &str) -> usize {
    let mut answer = std::usize::MAX;

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
    'outer: for c in ['-', '|', 'L', 'J', '7', 'F'] {
        let mut start = true;

        let mut sx = s_x;
        let mut sy = s_y;
        let mut length = 0;
        grid[sx][sy] = c;

        let mut direction = Direction::Unknown;

        loop {
            if !start && (sx == s_x && sy == s_y) {
                break;
            }
            start = false;

            if length >= answer {
                continue 'outer;
            }

            // what we need to figure out the next step
            // 1. The current character
            // 2. Direction we are headed in
            // 3. the potential next character. since it's possible we jump to an un jumpable
            //    character

            let (new_direction, valid_pipes) = next_direction(grid[sx][sy], &direction);

            let (x, y) = match new_direction {
                Direction::Unknown => unreachable!(),
                Direction::North => (-1, 0),
                Direction::South => (1, 0),
                Direction::East => (0, 1),
                Direction::West => (0, -1),
            };

            let p = sx as i32 + x;
            let q = sy as i32 + y;
            if p < 0 || q < 0 || p >= m as i32 || q >= n as i32 {
                continue 'outer;
            }

            let next_pipe = grid[p as usize][q as usize];
            if valid_pipes.contains(&next_pipe) {
                sx = p as usize;
                sy = q as usize;
                direction = new_direction;
                length += 1;
            } else {
                continue 'outer;
            }
        }

        answer = std::cmp::min(answer, length / 2);
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

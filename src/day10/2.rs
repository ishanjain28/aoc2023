#![feature(test)]

extern crate test;

const INPUTS: [&str; 5] = [
    "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
    "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........",
    "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
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

    'outer: for c in ['L', 'F', '-', '|', 'J', '7'] {
        let mut start = true;
        grid[s_x][s_y] = c;
        let mut sx = s_x;
        let mut sy = s_y;

        let mut direction = Direction::Unknown;
        let mut local_map = map.clone();

        loop {
            if !start && (sx == s_x && sy == s_y) {
                break;
            }
            start = false;
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
                local_map[sx][sy] = grid[sx][sy];
                sx = p as usize;
                sy = q as usize;
                direction = new_direction;
            } else {
                continue 'outer;
            }
        }

        map = local_map;
    }

    for line in map.into_iter() {
        let mut vertical_lines = 0;

        let mut found_l = false;
        let mut found_f = false;
        for c in line {
            let count = match c {
                '|' => 1,
                'L' => {
                    found_l = true;
                    0
                }
                '7' if found_l => 1,
                'F' => {
                    found_f = true;
                    0
                }
                'J' if found_f => 1,
                '-' => 0,
                _ => {
                    found_l = false;
                    found_f = false;
                    0
                }
            };

            vertical_lines += count;
            if count > 0 {
                found_l = false;
                found_f = false;
            }

            if c == '.' && vertical_lines % 2 == 1 {
                answer += 1;
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

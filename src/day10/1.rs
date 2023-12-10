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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Vertical = 0x1,
    Horizontal = 0x2,
    L = 0x4,
    J = 0x8,
    Seven = 0x10,
    F = 0x20,
    Ground = 0x40,
    Start = 0x80,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;

        match value {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => L,
            'J' => J,
            '7' => Seven,
            'F' => F,
            '.' => Ground,
            'S' => Start,

            _ => unreachable!(),
        }
    }
}

fn next_direction(a: char, dir: &Direction) -> (Direction, u8) {
    use Direction::*;
    use Tile::*;
    match (a, dir) {
        ('|', Unknown | North) => (North, Vertical as u8 | Seven as u8 | F as u8),
        ('|', South) => (South, Vertical as u8 | L as u8 | J as u8),

        ('-', Unknown | West) => (West, Horizontal as u8 | L as u8 | F as u8),
        ('-', East) => (East, Horizontal as u8 | J as u8 | Seven as u8),

        ('L', Unknown | West) => (North, Vertical as u8 | Seven as u8 | F as u8),
        ('L', South) => (East, Horizontal as u8 | J as u8 | Seven as u8),

        ('J', Unknown | South) => (West, Horizontal as u8 | L as u8 | F as u8),
        ('J', East) => (North, Vertical as u8 | F as u8 | Seven as u8),

        ('7', Unknown | East) => (South, Vertical as u8 | L as u8 | J as u8),
        ('7', North) => (West, Horizontal as u8 | L as u8 | F as u8),

        ('F', Unknown | North) => (East, Horizontal as u8 | Seven as u8 | J as u8),
        ('F', West) => (South, Vertical as u8 | L as u8 | J as u8),

        v => {
            println!("{:?}", v);
            unreachable!()
        }
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

            let next_pipe = Tile::from(grid[p as usize][q as usize]);
            if valid_pipes & next_pipe as u8 > 0 {
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

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

    // Reusing a bitmap because these will never be togther in a single map
    X = 0x1 | 0x2,
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

    let mut map = vec![vec![Tile::Ground; n]; m];

    'outer: for c in ['-', '|', 'L', 'J', '7', 'F'] {
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

            local_map[sx][sy] = Tile::from(grid[sx][sy]);
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
            } else {
                continue 'outer;
            }
        }

        map = local_map;
    }

    let mut nmap = vec![vec![Tile::Ground; n * 2]; m * 2];

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            nmap[i * 2][j * 2] = *c;
        }
    }

    use Tile::*;
    for line in nmap.iter_mut() {
        for j in 0..2 * n - 2 {
            if (line[j] as u8 & (F as u8 | L as u8 | Horizontal as u8)) > 0
                && (line[j + 2] as u8 & (J as u8 | Seven as u8 | Horizontal as u8)) > 0
            {
                line[j + 1] = Horizontal;
            }
        }
    }

    for j in 0..n * 2 {
        for i in 0..m * 2 - 2 {
            if (nmap[i][j] as u8 & (F as u8 | Seven as u8 | Vertical as u8)) > 0
                && (nmap[i + 2][j] as u8 & (L as u8 | J as u8 | Vertical as u8)) > 0
            {
                nmap[i + 1][j] = Vertical;
            }
        }
    }

    for j in 0..2 * n {
        if nmap[0][j] == Ground {
            flood_fill(&mut nmap, 0, j, X);
        }
    }

    let mut out = vec![vec![Tile::Ground; n]; m];

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
            if c == Tile::Ground {
                answer += 1;
            }
        }
    }

    answer
}

fn flood_fill(map: &mut Vec<Vec<Tile>>, i: usize, j: usize, fill_with: Tile) {
    let m = map.len();
    let n = map[0].len();
    const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [-1, 0], [1, 0]];

    let mut stack = vec![];
    stack.push((i, j));

    while let Some((sx, sy)) = stack.pop() {
        if map[sx][sy] != Tile::Ground {
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
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

#![feature(test)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
];

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
            if !(b'1'..=b'9').contains(&grid[i][j]) {
                j += 1;
                continue;
            }

            let mut ey = j;
            while ey < n && grid[i][ey].is_ascii_digit() {
                ey += 1;
            }

            let mut valid = false;

            'outer: for (a, b) in DIRS.iter() {
                let x = i as i32 + a;

                if x < 0 || x >= m as i32 {
                    continue;
                }

                for p in j..ey {
                    let y = p as i32 + b;

                    if y < 0 || y >= n as i32 {
                        continue;
                    }

                    let c = grid[x as usize][y as usize];
                    if c != b'.' && !c.is_ascii_digit() {
                        valid = true;
                        break 'outer;
                    }
                }
            }

            if valid {
                total += parse(&grid[i][j..ey]);
            }

            j += ey - j;
        }
    }

    total
}

fn parse(b: &[u8]) -> u64 {
    let mut output = 0;

    for (i, v) in b.iter().enumerate() {
        output += 10u64.pow(b.len() as u32 - i as u32 - 1) * ((v - b'0') as u64);
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

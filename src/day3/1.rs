#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut total = 0;

    let grid: Vec<&[u8]> = data
        .split(|&x| x == b'\n')
        .filter(|x| !x.is_empty())
        .collect();

    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        let mut j = 0;
        while j < n {
            if (b'1'..=b'9').contains(&grid[i][j]) {
                let int: Vec<u8> = grid[i]
                    .iter()
                    .skip(j)
                    .take_while(|&c| c.is_ascii_digit())
                    .cloned()
                    .collect();

                let mut valid = false;

                let dirs = [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, -1),
                ];

                'outer: for (a, b) in dirs {
                    let x = i as i32 + a;

                    for p in j..j + int.len() {
                        let y = p as i32 + b;

                        if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
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
                    let num = String::from_utf8_lossy(&int).parse::<u64>().unwrap();
                    total += num
                }

                j += int.len();
            } else {
                j += 1;
            }
        }
    }

    total
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

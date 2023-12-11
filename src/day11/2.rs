#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

const MULTIPLIER: usize = 999999;

fn process(data: &str) -> usize {
    let grid: Vec<Vec<char>> = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();

    let n = grid[0].len();

    let mut blank_cols = Vec::new();
    for j in 0..n {
        let mut all_empty = true;

        for row in grid.iter() {
            if row[j] != '.' {
                all_empty = false;
                break;
            }
        }

        if all_empty {
            blank_cols.push(j);
        }
    }

    let mut nodes = vec![];

    let mut total_row_offset = 0;
    for (i, row) in grid.iter().enumerate() {
        if !row.iter().any(|&x| x == '#') {
            total_row_offset += MULTIPLIER;
        }
        let mut total_col_offset = 0;
        let mut j_offset = 0;

        for j in 0..n {
            if j_offset < blank_cols.len() && j >= blank_cols[j_offset] {
                total_col_offset += MULTIPLIER;
                j_offset += 1;
            }
            if grid[i][j] != '#' {
                continue;
            }

            nodes.push(((i + total_row_offset), (j + total_col_offset)));
        }
    }

    let mut answer = 0;
    for (i, &(sx, sy)) in nodes.iter().enumerate() {
        for &(dx, dy) in nodes.iter().skip(i + 1) {
            let distance = ((dy as i32 - sy as i32).abs() + (dx as i32 - sx as i32).abs()) as usize;

            answer += distance;
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

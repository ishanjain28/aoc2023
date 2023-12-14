#![feature(test)]

use std::collections::{hash_map::Entry, HashMap};

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let mut answer = 0;

    let mut grid: Vec<Vec<u8>> = data.lines().map(|x| x.bytes().collect()).collect();

    let m = grid.len();

    let mut map: HashMap<Vec<Vec<u8>>, Vec<Vec<u8>>> = HashMap::new();

    let mut steps_left = 1_000_000_000;

    while steps_left > 0 {
        let l = map.len();
        spin(&mut grid, &mut map);
        steps_left -= 1;
        if l == map.len() {
            break;
        }
    }

    let mut index_map: HashMap<&Vec<Vec<u8>>, usize> = HashMap::new();
    let mut path = vec![0; map.len()];

    let mut i = 0;
    for (k, v) in map.iter() {
        let k_idx: usize = match index_map.entry(k) {
            Entry::Occupied(v) => *v.get(),
            Entry::Vacant(e) => {
                e.insert(i);
                i += 1;
                i - 1
            }
        };

        let v_idx: usize = match index_map.entry(v) {
            Entry::Occupied(v) => *v.get(),
            Entry::Vacant(e) => {
                e.insert(i);
                i += 1;
                i - 1
            }
        };

        path[k_idx] = v_idx;
    }

    let mut current = *index_map.get(&grid).unwrap();

    let cycle_length = {
        let mut c = current;
        let mut start = true;
        let mut length = 0;

        while start || c != current {
            start = false;
            c = path[c];
            length += 1;
        }
        length
    };

    steps_left %= cycle_length;
    while steps_left > 0 {
        current = path[current];
        steps_left -= 1;
    }

    for (k, v) in index_map.iter() {
        if *v == current {
            for (i, line) in k.iter().enumerate() {
                for &c in line {
                    if c == b'O' {
                        answer += m - i;
                    }
                }
            }

            break;
        }
    }

    answer
}

fn spin(grid: &mut Vec<Vec<u8>>, map: &mut HashMap<Vec<Vec<u8>>, Vec<Vec<u8>>>) {
    let start = grid.clone();

    if let Some(v) = map.get(&start) {
        *grid = v.clone();
        return;
    }
    let m = grid.len();
    let n = grid[0].len();

    for i in 0..m {
        for j in 0..n {
            let c = grid[i][j];

            if c != b'O' {
                continue;
            }

            // Roll north
            let mut start = i;
            while start > 0 && grid[start - 1][j] == b'.' {
                start -= 1;
            }

            grid[i][j] = b'.';
            grid[start][j] = b'O';
        }
    }

    for row in grid.iter_mut() {
        for j in 0..n {
            let c = row[j];

            if c != b'O' {
                continue;
            }

            // Roll West
            let mut start = j;
            while start > 0 && row[start - 1] == b'.' {
                start -= 1;
            }

            row[j] = b'.';
            row[start] = b'O';
        }
    }

    for i in (0..m).rev() {
        for j in 0..n {
            let c = grid[i][j];

            if c != b'O' {
                continue;
            }

            // Roll South
            let mut start = i;
            while start < m - 1 && grid[start + 1][j] == b'.' {
                start += 1;
            }

            grid[i][j] = b'.';
            grid[start][j] = b'O';
        }
    }

    for row in grid.iter_mut() {
        for j in (0..n).rev() {
            let c = row[j];

            if c != b'O' {
                continue;
            }

            // Roll East
            let mut start = j;
            while start < n - 1 && row[start + 1] == b'.' {
                start += 1;
            }

            row[j] = b'.';
            row[start] = b'O';
        }
    }
    map.insert(start, grid.clone());
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[bench]
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

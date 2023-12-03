#![feature(test)]

use std::{collections::HashSet, ops::RangeInclusive};
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

    let mut ranges = Vec::with_capacity(m);

    for row in grid.iter() {
        let mut range = vec![];

        let mut sy = 0;
        let mut ey = 0;
        let mut counting = false;

        for j in 0..n {
            let c = row[j];

            if c.is_ascii_digit() {
                if !counting {
                    sy = j;
                    ey = j;
                    counting = true;
                } else {
                    ey += 1;
                }
            } else {
                if counting {
                    range.push(sy..=ey);
                }
                counting = false;
                sy = 0;
                ey = 0;
            }
        }

        if counting {
            range.push(sy..=ey);
        }

        ranges.push(range);
    }

    let mut set = HashSet::with_capacity(200);

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'.' || grid[i][j].is_ascii_digit() {
                continue;
            }
            for (p, q) in DIRS.iter() {
                let x = i as i32 + p;
                let y = j as i32 + q;

                if x < 0 || y < 0 || x > m as i32 || y > n as i32 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if let Some(range) = find(y, &ranges[x]) {
                    set.insert((x, range));
                }
            }
        }
    }

    set.into_iter()
        .map(|(x, range)| {
            String::from_utf8_lossy(&grid[x][range.clone()])
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>()

    //    let mut total = 0;
    //    let mut v2 = Vec::new();
    //
    //    for i in 0..m {
    //        let mut j = 0;
    //        while j < n {
    //            if !(b'1'..=b'9').contains(&grid[i][j]) {
    //                j += 1;
    //                continue;
    //            }
    //
    //            let mut ey = j;
    //            while ey < n && grid[i][ey].is_ascii_digit() {
    //                ey += 1;
    //            }
    //
    //            let mut valid = false;
    //
    //            'outer: for (a, b) in DIRS.iter() {
    //                let x = i as i32 + a;
    //
    //                for p in j..ey {
    //                    let y = p as i32 + b;
    //
    //                    if x < 0 || y < 0 || x >= m as i32 || y >= n as i32 {
    //                        continue;
    //                    }
    //
    //                    let c = grid[x as usize][y as usize];
    //                    if c != b'.' && !c.is_ascii_digit() {
    //                        valid = true;
    //                        break 'outer;
    //                    }
    //                }
    //            }
    //
    //            if valid {
    //                let num = String::from_utf8_lossy(&grid[i][j..ey])
    //                    .parse::<u64>()
    //                    .unwrap();
    //                v2.push(num);
    //                total += num
    //            }
    //
    //            j += ey - j;
    //        }
    //    }
    //
    //    for i in 0..v1.len() {
    //        if let Some(x) = v2.iter().position(|a| a == &v1[i]) {
    //            v2.remove(x);
    //        }
    //        v1[i] = 0;
    //    }
    //    for v in &v2 {
    //        if let Some(x) = v1.iter().position(|a| a == v) {
    //            v1.remove(x);
    //        }
    //    }
    //
    //    println!("{:?} {:?}", v1, v2);
    //    total
}

fn find(v: usize, r: &[RangeInclusive<usize>]) -> Option<&RangeInclusive<usize>> {
    let mut start = 0;
    let mut end = r.len();

    while start < end {
        let mid = start + (end - start) / 2;
        let mid_v = &r[mid];

        if mid_v.contains(&v) {
            return Some(mid_v);
        }

        if mid_v.start() > &v {
            end = mid;
        }
        if mid_v.end() < &v {
            start = mid + 1;
        }
    }

    None
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

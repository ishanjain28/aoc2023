#![feature(test)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> usize {
    let mut size = 1;
    let mut i = 0;
    while i < data.len() - 1 {
        if data[i] == b'\n' {
            i += (data[i + 1] == b'\n') as usize;
            size += 1;
        }
        i += 1;
    }

    let input: Vec<u8> = data.iter().filter(|&&x| x != b'\n').cloned().collect();

    let mut total_row_offset = 0;
    let mut total_col_offset = vec![0; size];
    let mut nodes = vec![];

    for i in 0..size {
        if !input[size * i..size * i + size].iter().any(|&x| x == b'#') {
            total_row_offset += 1;
        }

        for j in 0..size {
            if total_col_offset[j] == 0
                && !input[j..size * size]
                    .iter()
                    .step_by(size)
                    .any(|&x| x != b'.')
            {
                total_col_offset[j] = total_col_offset[j - 1] + 1;
            } else if j > 0 && total_col_offset[j] == 0 {
                total_col_offset[j] += total_col_offset[j - 1];
            }

            if input[i * size + j] != b'#' {
                continue;
            }

            nodes.push((i + total_row_offset, j + total_col_offset[j]));
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

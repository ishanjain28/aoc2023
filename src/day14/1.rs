#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> usize {
    let size = data.lines().count();

    let mut data: Vec<u8> = data.bytes().collect();
    let mut answer = 0;

    for i in 0..size {
        for j in 0..size {
            let c = unsafe { *data.get_unchecked(size * i + j + i) };

            if c != b'O' {
                continue;
            }

            // move the rock north
            let mut start = i;
            while start > 0
                && unsafe { *data.get_unchecked(size * (start - 1) + j + start - 1) } == b'.'
            {
                start -= 1;
            }

            unsafe {
                *data.get_unchecked_mut(size * i + j + i) = b'.';
                *data.get_unchecked_mut(size * start + j + start) = b'O';
            }
        }
    }

    for i in 0..size {
        let line = unsafe { data.get_unchecked(size * i + i..size * i + i + size) };
        let count: usize = line.iter().filter(|&&x| x == b'O').count();

        answer += count * (size - i);
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

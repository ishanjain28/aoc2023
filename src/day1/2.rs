#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> u64 {
    let mut total: u64 = 0;

    for line in data.split('\n') {
        let mut first = 0;
        let mut last = 0;

        let mut window = Vec::with_capacity(line.len());
        let mut j = 0;

        for (i, c) in line.chars().enumerate() {
            if (window.len() - j) >= 5 {
                j += 1;
            }

            match c {
                v @ '1'..='9' if first == 0 => {
                    let num = v as u8 - b'0';
                    first = 10 * num;
                    last = num;

                    j = i + 1;
                }
                v @ '1'..='9' => {
                    let num = v as u8 - b'0';
                    last = num;

                    j = i + 1;
                }
                _ => (),
            }
            window.push(c);

            if window.len() - j >= 3 {
                if let Some(digit) = get_digit(&window[j..]) {
                    if first == 0 {
                        first = 10 * digit;
                    }
                    last = digit;
                }
            }
        }

        if let Some(digit) = get_digit(&window[j..]) {
            if first == 0 {
                first = 10 * digit;
            }
            last = digit;
        }

        total += first as u64 + last as u64;
    }

    total
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[inline]
const fn get_digit(set: &[char]) -> Option<u8> {
    match set {
        ['t', 'h', 'r', 'e', 'e'] => Some(3),
        ['s', 'e', 'v', 'e', 'n'] => Some(7),
        ['e', 'i', 'g', 'h', 't'] => Some(8),
        ['o', 'n', 'e', _, _] | ['o', 'n', 'e'] | [_, 'o', 'n', 'e'] | [_, _, 'o', 'n', 'e'] => {
            Some(1)
        }
        ['t', 'w', 'o', _, _] | ['t', 'w', 'o'] | [_, 't', 'w', 'o'] | [_, _, 't', 'w', 'o'] => {
            Some(2)
        }
        ['s', 'i', 'x', _, _] | ['s', 'i', 'x'] | [_, 's', 'i', 'x'] | [_, _, 's', 'i', 'x'] => {
            Some(6)
        }
        ['n', 'i', 'n', 'e', _] | ['n', 'i', 'n', 'e'] | [_, 'n', 'i', 'n', 'e'] => Some(9),
        ['f', 'o', 'u', 'r', _] | ['f', 'o', 'u', 'r'] | [_, 'f', 'o', 'u', 'r'] => Some(4),
        ['f', 'i', 'v', 'e', _] | ['f', 'i', 'v', 'e'] | [_, 'f', 'i', 'v', 'e'] => Some(5),
        _ => None,
    }
}

#[bench]
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

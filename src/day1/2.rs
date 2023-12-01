#![feature(test)]
extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> u64 {
    let mut total: u64 = 0;

    for line in data.split('\n') {
        let line: Vec<char> = line.chars().collect();

        let first = 10 * find_forward(&line);
        let last = find_backward(&line);

        total += first as u64 + last as u64;
    }

    total
}

fn find_forward(iter: &[char]) -> u8 {
    let mut start = 0;
    let mut end = 0;

    for c in iter.iter() {
        if ('1'..='9').contains(c) {
            return *c as u8 - b'0';
        }

        if end - start >= 5 {
            start += 1;
        }
        end += 1;

        if end - start >= 3 {
            if let Some(digit) = get_digit(&iter[start..end]) {
                return digit;
            }
        }
    }

    get_digit(&iter[start..end]).unwrap_or(0)
}
fn find_backward(iter: &[char]) -> u8 {
    if iter.is_empty() {
        return 0;
    }
    let mut start = iter.len() - 1;
    let mut end = iter.len();

    for c in iter.iter().rev() {
        if ('1'..='9').contains(c) {
            return *c as u8 - b'0';
        }
        if end - start >= 5 {
            end -= 1;
        }

        start -= 1;

        if end - start >= 3 {
            if let Some(digit) = get_digit(&iter[start..end]) {
                return digit;
            }
        }
    }

    get_digit(&iter[start..end]).unwrap_or(0)
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

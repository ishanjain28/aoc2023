#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut total: u64 = 0;

    for line in data.split(|&x| x == b'\n') {
        let first = 10 * find_forward(line);
        let last = find_backward(line);

        total += first as u64 + last as u64;
    }

    total
}

fn find_forward(iter: &[u8]) -> u8 {
    let mut start = 0;
    let mut end = 0;

    for c in iter.iter() {
        if (b'1'..=b'9').contains(c) {
            return c - b'0';
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
fn find_backward(iter: &[u8]) -> u8 {
    if iter.is_empty() {
        return 0;
    }
    let mut start = iter.len() - 1;
    let mut end = iter.len();

    for c in iter.iter().rev() {
        if (b'1'..=b'9').contains(c) {
            return c - b'0';
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
const fn get_digit(set: &[u8]) -> Option<u8> {
    match set {
        [b't', b'h', b'r', b'e', b'e'] => Some(3),
        [b's', b'e', b'v', b'e', b'n'] => Some(7),
        [b'e', b'i', b'g', b'h', b't'] => Some(8),

        [b'o', b'n', b'e', _, _]
        | [b'o', b'n', b'e']
        | [_, b'o', b'n', b'e']
        | [_, _, b'o', b'n', b'e'] => Some(1),

        [b't', b'w', b'o', _, _]
        | [b't', b'w', b'o']
        | [_, b't', b'w', b'o']
        | [_, _, b't', b'w', b'o'] => Some(2),

        [b's', b'i', b'x', _, _]
        | [b's', b'i', b'x']
        | [_, b's', b'i', b'x']
        | [_, _, b's', b'i', b'x'] => Some(6),

        [b'n', b'i', b'n', b'e', _] | [b'n', b'i', b'n', b'e'] | [_, b'n', b'i', b'n', b'e'] => {
            Some(9)
        }
        [b'f', b'o', b'u', b'r', _] | [b'f', b'o', b'u', b'r'] | [_, b'f', b'o', b'u', b'r'] => {
            Some(4)
        }
        [b'f', b'i', b'v', b'e', _] | [b'f', b'i', b'v', b'e'] | [_, b'f', b'i', b'v', b'e'] => {
            Some(5)
        }
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

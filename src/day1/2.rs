use std::collections::VecDeque;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn main() {
    for input in INPUTS.iter() {
        let mut total: u64 = 0;

        for line in input.split('\n') {
            let line: Vec<char> = line.chars().collect();

            let mut first = None;
            let mut last = 0;

            let mut tmp: VecDeque<char> = VecDeque::new();

            for c in line.iter() {
                if tmp.len() >= 5 {
                    tmp.pop_front();
                }

                match c {
                    '1' => {
                        if first.is_none() {
                            first = Some(10 * 1);
                        }
                        last = 1;

                        tmp.clear();
                    }
                    '2' => {
                        if first.is_none() {
                            first = Some(10 * 2);
                        }
                        last = 2;
                        tmp.clear();
                    }
                    '3' => {
                        if first.is_none() {
                            first = Some(10 * 3);
                        }
                        last = 3;
                        tmp.clear();
                    }
                    '4' => {
                        if first.is_none() {
                            first = Some(10 * 4);
                        }
                        last = 4;
                        tmp.clear();
                    }
                    '5' => {
                        if first.is_none() {
                            first = Some(10 * 5);
                        }
                        last = 5;
                        tmp.clear();
                    }
                    '6' => {
                        if first.is_none() {
                            first = Some(10 * 6);
                        }
                        last = 6;
                        tmp.clear();
                    }
                    '7' => {
                        if first.is_none() {
                            first = Some(10 * 7);
                        }
                        last = 7;
                        tmp.clear();
                    }
                    '8' => {
                        if first.is_none() {
                            first = Some(10 * 8);
                        }
                        last = 8;
                        tmp.clear();
                    }
                    '9' => {
                        if first.is_none() {
                            first = Some(10 * 9);
                        }
                        last = 9;
                        tmp.clear();
                    }

                    c => {
                        tmp.push_back(*c);
                    }
                }

                if let Some(digit) = get_digit(tmp.make_contiguous()) {
                    if first.is_none() {
                        first = Some(10 * digit);
                    }
                    last = digit;
                }
            }

            if let Some(digit) = get_digit(tmp.make_contiguous()) {
                if first.is_none() {
                    first = Some(10 * digit);
                }
                last = digit;
            }

            total += first.unwrap_or(0) as u64 + last as u64;
        }

        println!("total = {}", total);
    }
}

fn get_digit(set: &[char]) -> Option<u8> {
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

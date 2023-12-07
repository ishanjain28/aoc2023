#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::cmp::Ordering;

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug)]
struct Card<'a> {
    val: &'a [u8],
    set_type: SetType,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SetType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    Two = 3,
    One = 2,
    High = 1,
}

const ARRAY_SIZE: usize = 15;

fn process(data: &[u8]) -> u64 {
    let mut answer = 0;
    let mut cards = vec![];

    for data in data.split(|&x| x == b'\n') {
        if data.is_empty() {
            continue;
        }
        let (card, value) = data.split_at(6);
        let card = card.trim_ascii();

        let mut arr = [0; ARRAY_SIZE];

        for &c in card {
            let c = card_weight(c);
            arr[c as usize] += 1;
        }
        let transposed = transpose(&arr);

        let set_type = match transposed {
            [_, _, _, _, _, 1] => SetType::Five,

            [_, 1, _, _, 1, _] => SetType::Four,

            [_, _, 1, 1, _, _] => SetType::FullHouse,

            [_, 2, _, 1, _, _] => SetType::Three,

            [_, 1, 2, _, _, _] => SetType::Two,

            [_, 3, 1, _, _, _] => SetType::One,

            [_, 5, _, _, _, _] => SetType::High,

            _ => unreachable!(),
        };

        let mut num = 0;
        let mut pow = 1;

        for &val in value.iter().rev() {
            num += (val - b'0') as u64 * pow;
            pow *= 10;
        }

        cards.push((
            Card {
                val: card,
                set_type,
            },
            num,
        ));
    }

    cards.sort_unstable_by(|(a, _), (b, _)| match (a.set_type, b.set_type) {
        (x, y) if x == y => {
            for (a, b) in a.val.iter().zip(b.val.iter()) {
                let ordering = cmp(*a, *b);

                if ordering != Ordering::Equal {
                    return ordering;
                }
            }

            Ordering::Equal
        }
        (x, y) => {
            let x = x as u8;
            let y = y as u8;

            x.cmp(&y)
        }
    });

    for (i, (_, v)) in cards.into_iter().enumerate() {
        let i = i + 1;

        answer += v * i as u64;
    }

    answer
}

#[inline]
const fn cmp(a: u8, b: u8) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }

    match (a, b) {
        (b'A', _) => Ordering::Greater,
        (_, b'A') => Ordering::Less,

        (b'K', _) => Ordering::Greater,
        (_, b'K') => Ordering::Less,

        (b'Q', _) => Ordering::Greater,
        (_, b'Q') => Ordering::Less,

        (b'J', _) => Ordering::Greater,
        (_, b'J') => Ordering::Less,

        (b'T', _) => Ordering::Greater,
        (_, b'T') => Ordering::Less,

        (b'9', _) => Ordering::Greater,
        (_, b'9') => Ordering::Less,

        (b'8', _) => Ordering::Greater,
        (_, b'8') => Ordering::Less,

        (b'7', _) => Ordering::Greater,
        (_, b'7') => Ordering::Less,

        (b'6', _) => Ordering::Greater,
        (_, b'6') => Ordering::Less,

        (b'5', _) => Ordering::Greater,
        (_, b'5') => Ordering::Less,

        (b'4', _) => Ordering::Greater,
        (_, b'4') => Ordering::Less,

        (b'3', _) => Ordering::Greater,
        (_, b'3') => Ordering::Less,

        (b'2', _) => Ordering::Greater,
        (_, b'2') => Ordering::Less,

        (_, _) => unreachable!(),
    }
}

#[inline]
fn transpose(ip: &[u8; ARRAY_SIZE]) -> [u8; 6] {
    let mut out = [0; 6];

    for &v in ip {
        out[v as usize] += 1;
    }

    out
}

#[inline]
const fn card_weight(a: u8) -> u8 {
    match a {
        b'2'..=b'9' => a - b'0',

        b'J' => 10,
        b'T' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,

        _ => 0,
    }
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

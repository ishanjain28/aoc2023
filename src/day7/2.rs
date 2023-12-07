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
    ctype: SetType,
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

const ARRAY_SIZE: usize = 14;

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

        for &c in card.iter() {
            let c = card_weight(c);
            arr[c as usize] += 1;
        }

        let count = arr[card_weight(b'J') as usize];
        let transposed = transpose(&arr);

        let ctype = match (count, transposed) {
            (_, [_, _, _, _, _, 1]) => SetType::Five,

            (4, [_, 1, _, _, 1, _]) => SetType::Five,
            (1, [_, 1, _, _, 1, _]) => SetType::Five,
            (_, [_, 1, _, _, 1, _]) => SetType::Four,

            (3, [_, _, 1, 1, _, _]) => SetType::Five,
            (2, [_, _, 1, 1, _, _]) => SetType::Five,
            (_, [_, _, 1, 1, _, _]) => SetType::FullHouse,

            (3, [_, 2, _, 1, _, _]) => SetType::Four,
            (1, [_, 2, _, 1, _, _]) => SetType::Four,
            (_, [_, 2, _, 1, _, _]) => SetType::Three,

            (2, [_, 1, 2, _, _, _]) => SetType::Four,
            (1, [_, 1, 2, _, _, _]) => SetType::FullHouse,
            (_, [_, 1, 2, _, _, _]) => SetType::Two,

            (2, [_, 3, 1, _, _, _]) => SetType::Three,
            (1, [_, 3, 1, _, _, _]) => SetType::Three,
            (_, [_, 3, 1, _, _, _]) => SetType::One,

            (1, [_, 5, _, _, _, _]) => SetType::One,
            (_, [_, 5, _, _, _, _]) => SetType::High,

            (_, _) => unreachable!(),
        };

        let mut num = 0;
        let mut pow = 1;

        for &val in value.iter().rev() {
            num += (val - b'0') as u64 * pow;
            pow *= 10;
        }

        cards.push((Card { val: card, ctype }, num));
    }

    cards.sort_unstable_by(|(a, _), (b, _)| match (a.ctype, b.ctype) {
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
        b'J' => 1,

        b'2'..=b'9' => a - b'0',

        b'T' => 10,
        b'Q' => 11,
        b'K' => 12,
        b'A' => 13,

        _ => 0,
    }
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

        (b'J', _) => Ordering::Greater,
        (_, b'J') => Ordering::Less,

        (_, _) => unreachable!(),
    }
}

fn main() {
    for input in INPUTS.iter() {
        println!("total = {}", process(input));
    }
}

#[bench]
fn part2(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[1]);
        test::black_box(v);
    });
}

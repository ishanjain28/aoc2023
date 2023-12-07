#![feature(byte_slice_trim_ascii)]
#![feature(test)]

use std::cmp::Ordering;

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

#[derive(Debug)]
struct Card {
    val: usize,
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

fn process(data: &[u8]) -> usize {
    let mut answer = 0;
    let mut cards = vec![];

    for data in data.split(|&x| x == b'\n') {
        if data.is_empty() {
            continue;
        }
        let (card, value) = data.split_at(5);
        let value = &value[1..];

        let mut arr = [0; ARRAY_SIZE];

        for &c in card {
            let c = card_weight(c);
            arr[c] += 1;
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
            num += (val - b'0') as usize * pow;
            pow *= 10;
        }

        let card = (card_weight(card[0]) << 32)
            + (card_weight(card[1]) << 24)
            + (card_weight(card[2]) << 16)
            + (card_weight(card[3]) << 8)
            + card_weight(card[4]);

        cards.push((
            Card {
                val: card,
                set_type,
            },
            num,
        ));
    }

    cards.sort_unstable_by(|(a, _), (b, _)| -> Ordering {
        match (a.set_type, b.set_type) {
            (x, y) if x == y => a.val.cmp(&b.val),
            (x, y) => {
                let x = x as u8;
                let y = y as u8;

                x.cmp(&y)
            }
        }
    });

    for (i, (_, v)) in cards.into_iter().enumerate() {
        let i = i + 1;

        answer += v * i;
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
const fn card_weight(a: u8) -> usize {
    match a {
        b'2'..=b'9' => (a - b'0') as usize,

        b'T' => 10,
        b'J' => 11,
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

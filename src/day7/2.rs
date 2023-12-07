#![feature(test)]

use std::cmp::Ordering;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug)]
struct Card {
    val: String,
    ttype: SetType,
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

fn process(data: &str) -> u64 {
    let mut answer = 0;

    let mut cards = vec![];

    for data in data.lines() {
        if data.is_empty() {
            continue;
        }
        let (card, value) = data.split_at(6);
        let card = card.trim();

        let mut arr = [0; 255];

        for c in card.bytes() {
            arr[c as usize] += 1;
        }
        let count = arr['J' as u8 as usize];

        let ttype = if arr.iter().any(|&x| x == 5) {
            SetType::Five
        } else if arr.iter().any(|&x| x == 4) && arr.iter().any(|&x| x == 1) {
            if count == 4 || count == 1 {
                SetType::Five
            } else {
                SetType::Four
            }
        } else if arr.iter().any(|&x| x == 3) && arr.iter().any(|&x| x == 2) {
            if count == 3 || count == 2 {
                SetType::Five
            } else {
                SetType::FullHouse
            }
        } else if arr.iter().any(|&x| x == 3) && arr.iter().filter(|&&x| x == 1).count() == 2 {
            if count == 3 || count == 1 {
                SetType::Four
            } else {
                SetType::Three
            }
        } else if arr.iter().filter(|&&x| x == 2).count() == 2
            && arr.iter().filter(|&&x| x == 1).count() == 1
        {
            if count == 2 {
                SetType::Four
            } else if count == 1 {
                SetType::FullHouse
            } else {
                SetType::Two
            }
        } else if arr.iter().filter(|&&x| x == 2).count() == 1
            && arr.iter().filter(|&&x| x == 1).count() == 3
        {
            if count == 2 || count == 1 {
                SetType::Three
            } else {
                SetType::One
            }
        } else if arr.iter().all(|&x| x == 0 || x == 1) {
            if count == 1 {
                SetType::One
            } else {
                SetType::High
            }
        } else {
            unreachable!()
        };

        cards.push((
            Card {
                val: card.to_string(),
                ttype,
            },
            value.parse::<u64>().unwrap(),
        ));
    }
    cards.sort_unstable_by(|(a, _), (b, _)| match (a.ttype, b.ttype) {
        (x, y) if x == y => {
            for (a, b) in a.val.chars().zip(b.val.chars()) {
                let ordering = cmp(a, b);

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

fn cmp(a: char, b: char) -> Ordering {
    if let Ordering::Equal = a.cmp(&b) {
        return Ordering::Equal;
    }

    match (a, b) {
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,

        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,

        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,

        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,

        ('9', _) => Ordering::Greater,
        (_, '9') => Ordering::Less,

        ('8', _) => Ordering::Greater,
        (_, '8') => Ordering::Less,

        ('7', _) => Ordering::Greater,
        (_, '7') => Ordering::Less,

        ('6', _) => Ordering::Greater,
        (_, '6') => Ordering::Less,

        ('5', _) => Ordering::Greater,
        (_, '5') => Ordering::Less,

        ('4', _) => Ordering::Greater,
        (_, '4') => Ordering::Less,

        ('3', _) => Ordering::Greater,
        (_, '3') => Ordering::Less,

        ('2', _) => Ordering::Greater,
        (_, '2') => Ordering::Less,

        ('J', _) => Ordering::Greater,
        (_, 'J') => Ordering::Less,

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

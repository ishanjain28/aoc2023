#![feature(test)]
#![feature(slice_split_once)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> usize {
    let mut cards = vec![];

    for data in data.split(|&x| x == b'\n') {
        if data.is_empty() {
            continue;
        }
        let (x, nums) = data.split_once(|&x| x == b':').unwrap();

        let id = parse(&x[5..]);

        let (nums, wins) = nums.split_once(|&x| x == b'|').unwrap();
        let nums = nums.split(|&x| x == b' ');
        let wins = wins.split(|&x| x == b' ');

        let mut card = BitMap::new();
        let mut win = BitMap::new();

        for num in nums {
            if num.is_empty() {
                continue;
            }

            let num = parse(num);

            card.set(num);
        }

        for w in wins {
            if w.is_empty() {
                continue;
            }
            let num = parse(w);
            win.set(num);
        }

        cards.push((id, card, win));
    }

    let mut counter = vec![1; cards.len() + 1];
    counter[0] = 0;

    for (id, card, wins) in cards.into_iter() {
        let mut sum = 0;
        for c in 0..100 {
            if card.get(c) && wins.get(c) {
                sum += 1;
            }
        }

        let cid = counter[id];
        for c in counter.iter_mut().skip(id + 1).take(sum) {
            *c += cid;
        }
    }

    counter.into_iter().sum::<usize>()
}

#[derive(Clone)]
struct BitMap(u128);
impl BitMap {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }
    #[inline]
    pub fn set(&mut self, idx: usize) {
        debug_assert!(idx < 128);
        self.0 |= 1 << idx;
    }
    #[inline]
    pub const fn get(&self, idx: usize) -> bool {
        debug_assert!(idx < 128);
        (self.0 >> idx) & 1 == 1
    }
}

#[inline]
fn parse(b: &[u8]) -> usize {
    let mut out = 0;

    let mut pow = 1;
    for c in b.iter().rev() {
        if !c.is_ascii_digit() {
            continue;
        }
        out += (c - b'0') as usize * pow;
        pow *= 10;
    }

    out
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

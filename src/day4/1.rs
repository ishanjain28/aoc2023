#![feature(slice_split_once)]
#![feature(test)]
extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut total = 0;

    for data in data.split(|&x| x == b'\n') {
        if data.is_empty() {
            continue;
        }

        let (_, nums) = data.split_once(|&x| x == b':').unwrap();

        let (nums, wins) = nums.split_once(|&x| x == b'|').unwrap();

        let mut bit_map = BitMap::new();

        for win in wins.split(|&x| x == b' ') {
            if win.is_empty() {
                continue;
            }
            let win = parse(win);

            bit_map.set(win);
        }

        let mut val = 0;
        for num in nums.split(|&x| x == b' ') {
            if num.is_empty() {
                continue;
            }
            let num = parse(num);

            if bit_map.get(num) {
                if val == 0 {
                    val = 1;
                } else {
                    val *= 2;
                }
            }
        }

        total += val;
    }

    total
}

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

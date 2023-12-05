#![feature(test)]
#![feature(slice_split_once)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> u64 {
    let mut counter = vec![0];

    let mut colon_pos = 0;
    let mut vert_tab_pos = 0;

    for data in data.split(|&x| x == b'\n') {
        if data.is_empty() {
            continue;
        }
        if colon_pos == 0 || vert_tab_pos == 0 {
            colon_pos = data.iter().position(|&x| x == b':').unwrap();
            vert_tab_pos = data.iter().position(|&x| x == b'|').unwrap();
        }

        let (x, nums) = data.split_at(colon_pos + 2);

        let id = parse(&x[4..x.len() - 2]) as usize;

        let (nums, wins) = nums.split_at(vert_tab_pos + 1 - colon_pos - 2);
        let mut win = BitMap::new();

        for c in wins.chunks_exact(3) {
            let num = parse(&c[1..3]);
            win.set(num);
        }

        let sum = nums
            .chunks_exact(3)
            .map(|x| parse(&x[0..2]))
            .fold(0, |a, x| a + win.get(x) as usize);

        if id + 1 + sum >= counter.len() {
            counter.extend(std::iter::repeat(1).take(id + 1 + sum - counter.len()));
        }

        let cid = counter[id];
        for c in counter.iter_mut().skip(id + 1).take(sum) {
            *c += cid;
        }
    }

    counter.into_iter().sum::<u64>()
}

#[derive(Debug, Clone)]
struct BitMap(u128);
impl BitMap {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }
    #[inline]
    pub fn set(&mut self, idx: u8) {
        debug_assert!(idx < 128);
        self.0 |= 1 << idx;
    }
    #[inline]
    pub const fn get(&self, idx: u8) -> bool {
        debug_assert!(idx < 128);
        (self.0 >> idx) & 1 == 1
    }
}

#[inline]
const fn parse(b: &[u8]) -> u8 {
    match b.len() {
        2 => 10 * (b[0] & 0xf) + (b[1] & 0xf),
        4 => 100 * (b[1] & 0xf) + 10 * (b[2] & 0xf) + (b[3] & 0xf),
        _ => unreachable!(),
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

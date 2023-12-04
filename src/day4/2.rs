#![feature(test)]
#![feature(slice_split_once)]

extern crate test;

const INPUTS: [&[u8]; 2] = [
    include_bytes!("./sample.txt"),
    include_bytes!("./input.txt"),
];

fn process(data: &[u8]) -> usize {
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

        let id = parse(&x[4..x.len() - 2]);

        let (nums, wins) = nums.split_at(vert_tab_pos + 1 - colon_pos - 2);

        let nums = nums[..nums.len() - 2].split(|&x| x == b' ');
        let wins = wins.split(|&x| x == b' ');

        let mut win = BitMap::new();

        for w in wins {
            if w.is_empty() {
                continue;
            }
            let num = parse(w);
            win.set(num);
        }

        let sum = nums
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(parse)
            .fold(0, |a, x| a + win.get(x) as usize);

        if id + 1 + sum >= counter.len() {
            counter.extend(std::iter::repeat(1).take(id + 1 + sum - counter.len()));
        }

        let cid = counter[id];
        for c in counter.iter_mut().skip(id + 1).take(sum) {
            *c += cid;
        }
    }

    counter.into_iter().sum::<usize>()
}

#[derive(Debug, Clone)]
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
            break;
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

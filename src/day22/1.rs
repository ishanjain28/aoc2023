#![feature(test)]

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Cuboid {
    start: (i32, i32, i32),
    end: (i32, i32, i32),
}

impl Cuboid {
    fn intersect(&self, other: Self) -> bool {
        let cs = self.start;
        let ce = self.end;
        let es = other.start;
        let ee = other.end;
        let min_x1 = cs.0.min(ce.0);
        let min_y1 = cs.1.min(ce.1);
        let min_z1 = cs.2.min(ce.2);

        let max_x1 = cs.0.max(ce.0);
        let max_y1 = cs.1.max(ce.1);
        let max_z1 = cs.2.max(ce.2);

        let min_x2 = es.0.min(ee.0);
        let min_y2 = es.1.min(ee.1);
        let min_z2 = es.2.min(ee.2);

        let max_x2 = es.0.max(ee.0);
        let max_y2 = es.1.max(ee.1);
        let max_z2 = es.2.max(ee.2);

        !((min_x1 > max_x2)
            || (max_x1 < min_x2)
            || (min_y1 > max_y2)
            || (max_y1 < min_y2)
            || (min_z1 > max_z2)
            || (max_z1 < min_z2))
    }
}

fn process(data: &str) -> i64 {
    let mut blocks = Vec::new();

    for line in data.lines() {
        let (start, end) = line.split_once('~').unwrap();

        let mut start = start.split(',');
        let sx = start.next().unwrap().parse::<i32>().unwrap();
        let sy = start.next().unwrap().parse::<i32>().unwrap();
        let sz = start.next().unwrap().parse::<i32>().unwrap();

        let mut end = end.split(',');
        let ex = end.next().unwrap().parse::<i32>().unwrap();
        let ey = end.next().unwrap().parse::<i32>().unwrap();
        let ez = end.next().unwrap().parse::<i32>().unwrap();

        blocks.push(Cuboid {
            start: (sx, sy, sz),
            end: (ex, ey, ez),
        });
    }

    blocks.sort_unstable_by(|a, b| a.start.2.cmp(&b.start.2));

    let blocks = drop(&blocks);

    count_safe_groups(&blocks)
}

fn drop(blocks: &[Cuboid]) -> Vec<Cuboid> {
    let mut current = blocks.to_vec();
    loop {
        let mut changed = false;
        let mut next: Vec<Cuboid> = vec![];
        for i in 0..current.len() {
            let block = &current[i];
            let mut new_block = *block;
            new_block.start.2 -= 1;
            new_block.end.2 -= 1;

            if new_block.start.2 < 1 {
                next.push(*block);
                continue;
            }

            let mut intersected = false;
            for other_block in current.iter().take(i) {
                if new_block.intersect(*other_block) {
                    intersected = true;
                    break;
                }
            }
            if intersected {
                next.push(*block);
            } else {
                next.push(new_block);
                changed = true
            }
        }
        current = next;
        if !changed {
            break;
        }
    }
    current
}
fn count_safe_groups(v: &Vec<Cuboid>) -> i64 {
    let mut out: i64 = 0;
    for i in 0..v.len() {
        let mut a = v.clone();
        a.remove(i);

        let dropped = drop(&a);
        if dropped == a {
            out += 1;
        }
    }
    out
}

fn main() {
    for input in INPUTS.iter().skip(1) {
        println!("answer = {}", process(input));
    }
}

#[bench]
fn part1(b: &mut test::Bencher) {
    b.iter(|| {
        let v = process(INPUTS[INPUTS.len() - 1]);
        test::black_box(v);
    });
}

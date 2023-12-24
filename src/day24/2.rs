#![feature(test)]

use z3::{
    ast::{Ast, Int},
    Config, Context, SatResult, Solver,
};

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> i64 {
    let config = Config::new();
    let ctx = Context::new(&config);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "X");
    let v_x = Int::new_const(&ctx, "V_X");
    let y = Int::new_const(&ctx, "Y");
    let v_y = Int::new_const(&ctx, "V_Y");
    let z = Int::new_const(&ctx, "Z");
    let v_z = Int::new_const(&ctx, "V_Z");
    let zero = Int::from_i64(&ctx, 0);

    let mut i = 0;
    for line in data.lines() {
        i += 1;
        let (a, b) = line.split_once(" @ ").unwrap();

        let pos: Vec<i64> = a
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let vel: Vec<i64> = b
            .split(',')
            .map(|x| x.trim())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let s_x = Int::from_i64(&ctx, pos[0]);
        let s_y = Int::from_i64(&ctx, pos[1]);
        let s_z = Int::from_i64(&ctx, pos[2]);

        let s_vx = Int::from_i64(&ctx, vel[0]);
        let s_vy = Int::from_i64(&ctx, vel[1]);
        let s_vz = Int::from_i64(&ctx, vel[2]);

        let t = Int::new_const(&ctx, i);

        // For every stone,
        // X + V_X * t == sx + s_vx * t;
        // Y + V_Y * t == sy + s_vy * t;
        // Z + V_Z * t == sz + s_vz * t;

        solver.assert(&t.ge(&zero));
        solver.assert(&((&x + &v_x * &t)._eq(&(&s_x + &s_vx * &t))));
        solver.assert(&((&y + &v_y * &t)._eq(&(&s_y + &s_vy * &t))));
        solver.assert(&((&z + &v_z * &t)._eq(&(&s_z + &s_vz * &t))));
    }

    let result = solver.check();
    if result != SatResult::Sat {
        unreachable!();
    }
    let model = solver.get_model().unwrap();
    let result = model.eval(&(&x + &y + &z), true).unwrap();

    result.as_i64().unwrap()
}

fn main() {
    for input in INPUTS.iter() {
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

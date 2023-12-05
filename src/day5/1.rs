#![feature(slice_split_once)]
#![feature(test)]

use std::ops::Range;

extern crate test;

const INPUTS: [&str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn process(data: &str) -> u64 {
    let mut lines = data.split("\n\n");

    let seeds: Vec<u64> = lines
        .next()
        .map(|x| x.split(' ').skip(1).map(|y| y.parse::<u64>().unwrap()))
        .unwrap()
        .collect();

    let seed_to_soil: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();

    let soil_to_ferti: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();

    let ferti_to_water: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();

    let water_to_light: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();

    let light_to_temp: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();

    let temp_to_humidity: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();
    let humidity_to_loc: Vec<(Range<u64>, Range<u64>)> = lines
        .next()
        .map(|x| {
            x.lines().skip(1).map(|line| {
                let y: Vec<u64> = line.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                let size = y[2];

                (y[1]..y[1] + size, y[0]..y[0] + size)
            })
        })
        .unwrap()
        .collect();

    let mut answer = std::u64::MAX;

    for seed in seeds {
        let soil = seed_to_soil
            .iter()
            .find(|x| x.0.contains(&seed))
            .map(|x| seed + x.1.start - x.0.start)
            .unwrap_or(seed);

        let ferti = soil_to_ferti
            .iter()
            .find(|x| x.0.contains(&soil))
            .map(|x| soil + x.1.start - x.0.start)
            .unwrap_or(soil);

        let water = ferti_to_water
            .iter()
            .find(|x| x.0.contains(&ferti))
            .map(|x| ferti + x.1.start - x.0.start)
            .unwrap_or(ferti);

        let light = water_to_light
            .iter()
            .find(|x| x.0.contains(&water))
            .map(|x| water + x.1.start - x.0.start)
            .unwrap_or(water);

        let temp = light_to_temp
            .iter()
            .find(|x| x.0.contains(&light))
            .map(|x| light + x.1.start - x.0.start)
            .unwrap_or(light);
        let humidity = temp_to_humidity
            .iter()
            .find(|x| x.0.contains(&temp))
            .map(|x| temp + x.1.start - x.0.start)
            .unwrap_or(temp);
        let loc = humidity_to_loc
            .iter()
            .find(|x| x.0.contains(&humidity))
            .map(|x| humidity + x.1.start - x.0.start)
            .unwrap_or(humidity);

        answer = std::cmp::min(answer, loc);
    }

    answer
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

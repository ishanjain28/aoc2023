use std::{env, fs::File, io};

const AOC_TOKEN: &str = include_str!("../.aoc_token");

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let day: u8 = args[1].parse::<u8>().expect("error in parsing day");
    assert!((1..=25).contains(&day));

    let request = ureq::get(&format!("https://adventofcode.com/2023/day/{}/input", day))
        .set("Cookie", AOC_TOKEN);
    println!("{:?}", request);

    let response = match request.call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error in fetching input for day {}: {}", day, e);

            if let Some(v) = e.into_response() {
                eprintln!(
                    "error in fetching input for day {} status code = {} response = {:?}",
                    day,
                    v.status(),
                    v.into_string()
                );
            }

            return;
        }
    };

    let mut file =
        File::create(format!("./src/day{}/input.txt", day)).expect("error in creating file");

    io::copy(&mut response.into_reader(), &mut file).expect("error in writing to file");

    println!("Saved input for day {}", day);
}

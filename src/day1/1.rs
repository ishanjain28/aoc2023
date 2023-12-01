const INPUTS: [&'static str; 2] = [include_str!("./sample.txt"), include_str!("./input.txt")];

fn main() {
    for input in INPUTS.iter() {
        let mut total = 0;
        for line in input.split('\n') {
            let first = line
                .chars()
                .find(|c| c.is_numeric())
                .map_or(0, |x| 10 * x.to_digit(10).unwrap());

            let last = line
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .map_or(0, |x| x.to_digit(10).unwrap());

            total += (first + last);
        }

        println!("total = {}", total);
    }
}

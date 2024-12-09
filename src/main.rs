mod day1;
mod day2;

use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day2.txt").unwrap();
    let result = day2::problem2(&input);

    println!("{result}");
}

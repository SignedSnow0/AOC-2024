mod day1;
mod day2;
mod day3;
mod day4;

use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    let result = day4::problem2(&input);

    println!("{result}");
}

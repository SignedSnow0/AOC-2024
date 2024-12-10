mod day1;
mod day2;
mod day3;

use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day3.txt").unwrap();
    let result = day3::problem1(&input);

    println!("{result}");
}

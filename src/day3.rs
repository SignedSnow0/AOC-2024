use std::{fmt::write, task::Wake};

use regex::Regex;

#[derive(Debug)]
enum Operation {
    Multiplication(u32, u32)
}

fn parse_input(input: &str) -> Vec<Operation> {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap(); 
    
    regex.find_iter(input).map_while(|c| {
        let entry = c.as_str();

        match &entry[..3] {
            "mul" => {
                let mut num_start =  entry.find('(').unwrap();              
                let mut num_end = entry.find(',').unwrap();

                let first_num = &entry[num_start+1..num_end].parse::<u32>().unwrap();

                num_start = num_end;
                num_end = entry.find(')').unwrap();

                let second_num = &entry[num_start+1..num_end].parse::<u32>().unwrap();

                Some(Operation::Multiplication(*first_num, *second_num))
            }
            _ => None
        }
    }).collect::<Vec<_>>() 
} 

pub fn problem1(input: &str) -> u32 {
    let operations = parse_input(input);

    operations.iter().map(|o| {
        match o {
            Operation::Multiplication(left, right) => left * right
        }
    }).sum()
}

pub fn problem2(input: &str) -> u32 {
    let regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\)").unwrap();
    let matches = regex.find_iter(input).map(|m| { m.as_str() }).collect::<Vec<_>>();

    let mut enabled = true;
    let mut operations: Vec<Operation> = Vec::new();
    for m in matches {
        if m == "do()" {
            enabled = true;
            continue;
        }
        if m == "don't()" {
            enabled = false;
            continue;
        }
       
        if !enabled {
            continue;
        }

        match &m[..3] {
            "mul" => {
                let mut num_start =  m.find('(').unwrap();              
                let mut num_end = m.find(',').unwrap();

                let first_num = &m[num_start+1..num_end].parse::<u32>().unwrap();

                num_start = num_end;
                num_end = m.find(')').unwrap();

                let second_num = &m[num_start+1..num_end].parse::<u32>().unwrap();

                operations.push(Operation::Multiplication(*first_num, *second_num));
            }
            _ => {
                continue;
            }
        }
    }

    operations.iter().map(|o| {
        match o {
            Operation::Multiplication(left, right) => left * right
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    const INPUT_FILE: &str = "./inputs/day3.txt";

    use super::*;
    use std::fs;

    #[test]
    fn test_problem1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = problem1(input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_problem2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = problem2(input);

        assert_eq!(result, 48);
    }

    #[test]
    fn test_problem1_input() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem1(&input);

        assert_eq!(result, 174336360);
    }

    #[test]
    fn test_problem2_input() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem2(&input);

        assert_eq!(result, 88802350);

    }
}

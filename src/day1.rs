fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
        input.lines()
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let item1 = line_iter.next().unwrap().parse::<u32>().unwrap();
            let item2 = line_iter.next().unwrap().parse::<u32>().unwrap();

            (item1, item2)
        })
        .unzip()
}

pub fn problem1(input: &str) -> u32 {
    let (mut list1, mut list2) = parse_input(input);
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(i1, i2)| i1.abs_diff(*i2))
        .sum::<u32>()
}

pub fn problem2(input: &str) -> u32 {
    let (list1, list2) = parse_input(input);

    list1.iter()
        .map(|i1| i1 * list2.iter().filter(|i2| i1 == *i2).count() as u32)
        .sum::<u32>()
}


#[cfg(test)]
mod tests {
    const INPUT_FILE: &str = "./inputs/day1.txt";

    use super::*;
    use std::fs;

    #[test]
    fn test_problem1() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3\n";

        let result = problem1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_problem2() {
        let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3\n";

        let result = problem2(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_problem1_file() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem1(&input);

        assert_eq!(result, 1830467);
    }

    #[test]
    fn test_problem2_file() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem2(&input);

        assert_eq!(result, 26674158);
    }
}

type Report = Vec<u32>;

fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn problem1(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .filter(|entry| {
            let distances = entry
                .windows(2)
                .map(|levels| levels[1] as i32 - levels[0] as i32)
                .collect::<Vec<_>>();

            let direction = distances.get(0).unwrap();

            distances
                .iter()
                .all(|dist| (dist.abs() >= 1 && dist.abs() <= 3) && dist * direction >= 0)
        })
        .count() as u32
}

pub fn problem2(input: &str) -> u32 {
    parse_input(input)
        .iter()
        .filter(|entry| {
            for i in 0..entry.len() {
                let distances = entry
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| if index == i { None } else { Some(item) })
                    .collect::<Vec<_>>()
                    .windows(2)
                    .map(|levels| *levels[1] as i32 - *levels[0] as i32)
                    .collect::<Vec<i32>>();

                let direction = distances.get(0).unwrap();

                if distances
                    .iter()
                    .all(|dist| (dist.abs() >= 1 && dist.abs() <= 3) && dist * direction > 0)
                {
                    return true;
                }
            }

            false
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    const INPUT_FILE: &str = "./inputs/day2.txt";

    use super::*;
    use std::fs;

    #[test]
    fn test_problem1() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";

        let result = problem1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_problem2() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";

        let result = problem2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_problem1_file() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem1(&input);

        assert_eq!(result, 202);
    }

    #[test]
    fn test_problem2_file() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem2(&input);

        assert_eq!(result, 271);
    }
}

fn get_from_matrix(input: &Vec<String>, positions: Vec<(usize, usize)>) -> Option<String> {
    let mut output = String::new();

    for position in positions {
        match input.get(position.0) {
            Some(line) => match line.chars().nth(position.1) {
                Some(char) => {
                    output.push(char);
                }
                None => return None,
            },
            None => return None,
        }
    }

    Some(output)
}

pub fn problem1(input: &str) -> u32 {
    let column_length = input.lines().count();

    let input = input.lines().map(String::from).collect::<Vec<_>>();

    let row_length = input.get(0).unwrap().len();

    let mut xmas_count = 0;
    for row in 0..row_length {
        for column in 0..column_length {
            match get_from_matrix(
                &input,
                vec![
                    (row, column),
                    (row, column + 1),
                    (row, column + 2),
                    (row, column + 3),
                ],
            ) {
                Some(value) => {
                    if value == "XMAS" || value == "SAMX" {
                        xmas_count += 1;
                    }
                }
                None => {}
            };

            match get_from_matrix(
                &input,
                vec![
                    (row, column),
                    (row + 1, column),
                    (row + 2, column),
                    (row + 3, column),
                ],
            ) {
                Some(value) => {
                    if value == "XMAS" || value == "SAMX" {
                        xmas_count += 1;
                    }
                }
                None => {}
            };

            match get_from_matrix(
                &input,
                vec![
                    (row, column),
                    (row + 1, column + 1),
                    (row + 2, column + 2),
                    (row + 3, column + 3),
                ],
            ) {
                Some(value) => {
                    if value == "XMAS" {
                        xmas_count += 1;
                    }
                }
                None => {}
            };

            if row >= 3 {
                match get_from_matrix(
                    &input,
                    vec![
                        (row, column),
                        (row - 1, column + 1),
                        (row - 2, column + 2),
                        (row - 3, column + 3),
                    ],
                ) {
                    Some(value) => {
                        if value == "XMAS" {
                            xmas_count += 1;
                        }
                    }
                    None => {}
                };
            }

            if column >= 3 {
                match get_from_matrix(
                    &input,
                    vec![
                        (row, column),
                        (row + 1, column - 1),
                        (row + 2, column - 2),
                        (row + 3, column - 3),
                    ],
                ) {
                    Some(value) => {
                        if value == "XMAS" {
                            xmas_count += 1;
                        }
                    }
                    None => {}
                };

            }

            if column >= 3 && row >= 3 {
                match get_from_matrix(
                    &input,
                    vec![
                        (row, column),
                        (row - 1, column - 1),
                        (row - 2, column - 2),
                        (row - 3, column - 3),
                    ],
                ) {
                    Some(value) => {
                        if value == "XMAS" {
                            xmas_count += 1;
                        }
                    }
                    None => {}
                };
            }
        }
    }

    xmas_count
}

pub fn problem2(input: &str) -> u32 {
    let column_length = input.lines().count();

    let input = input.lines().map(String::from).collect::<Vec<_>>();

    let row_length = input.get(0).unwrap().len();

    let mut xmas_count = 0;
    for row in 0..row_length {
        for column in 0..column_length {
            match get_from_matrix(
                &input,
                vec![
                    (row, column),
                    (row + 1, column + 1),
                    (row + 2, column + 2),
                ],
            ) {
                Some(value) => {
                    if value == "MAS" || value == "SAM" {
                        match get_from_matrix(
                            &input,
                            vec![
                                (row, column + 2),
                                (row + 1, column + 1),
                                (row + 2, column),
                            ],
                        ) {
                            Some(value) => {
                                if value == "MAS" || value == "SAM" {
                                    xmas_count += 1;
                                }
                            }
                            None => {}
                        };
                    }
                }
                None => {}
            };
        }
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    const INPUT_FILE: &str = "./inputs/day4.txt";

    use super::*;
    use std::fs;

    #[test]
    fn test_problem1() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        let result = problem1(input);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_problem2() {
        let input = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........\n";
        let result = problem2(input);

        assert_eq!(result, 9);
    }

    #[test]
    fn test_problem1_file() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem1(&input);

        assert_eq!(result, 2454);
    }

    #[test]
    fn test_problem2_file() {
        let input = fs::read_to_string(INPUT_FILE).unwrap();
        let result = problem2(&input);

        assert_eq!(result, 1858);
    }
}

use crate::util::read_by_line;

#[derive(Debug)]
enum Operator {
    ADD,
    MUL,
    NONE,
}

#[derive(Debug)]
struct Problem {
    operator: Operator,
    numbers: Vec<u64>,
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        if idx == 0 {
            for number in line.split_whitespace() {
                problems.push(Problem {
                    operator: Operator::NONE,
                    numbers: Vec::from([number.parse::<u64>().unwrap()]),
                })
            }
            continue;
        }
        for (idx, item) in line.split_whitespace().enumerate() {
            if item.parse::<u64>().is_err() {
                problems[idx].operator = match item {
                    "*" => Operator::MUL,
                    "+" => Operator::ADD,
                    _ => panic!("Unrecognized operator parse!")
                };
                continue;
            }
            problems[idx].numbers.push(item.parse::<u64>().unwrap());
        }
    }

    problems
}

fn solve_problem(problem: &Problem) -> u64 {
    match problem.operator {
        Operator::ADD => problem.numbers.iter().copied().reduce(|acc, x| acc + x).unwrap(),
        Operator::MUL => problem.numbers.iter().copied().reduce(|acc, x| acc * x).unwrap(),
        Operator::NONE => 0,
    }
}

pub fn run_challenge_06_00(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_06/".to_owned() + input_name));

    let problems = parse_problems(&input);

    let mut sum: u64 = 0;

    for problem in problems {
        sum += solve_problem(&problem);
    }

    sum
}

fn find_lexical_digit_ranges(input: &str) -> Vec<(usize, usize)> {
    let mut column_ranges: Vec<(usize, usize)> = Vec::new();

    for (line_idx, line) in input.lines().enumerate() {
        let mut current_column: usize = 0;
        let mut in_column: bool = false;
        for (idx, char) in line.chars().enumerate() {
            if char.is_whitespace() { 
                if in_column {
                    in_column = false;
                    current_column += 1;
                }
            }

            if char.is_numeric() {
                if !in_column {
                    in_column = true;
                    if (line_idx == 0) { column_ranges.push((idx, idx)); }
                    else { 
                        if (column_ranges[current_column].0 > idx) {
                            column_ranges[current_column].0 = idx;
                        }
                    }
                } else {
                    if column_ranges[current_column].1 < idx {
                        column_ranges[current_column].1 = idx;
                    }
                }
            }
        } 
    }

    column_ranges
}

fn parse_problems_v2(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut columns: Vec<Vec<Vec<char>>> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        if idx == 0 {
            for _ in line.split_whitespace() {
                problems.push(Problem {
                    numbers: Vec::new(),
                    operator: Operator::NONE,
                });
                columns.push(Vec::new());
            }
        }
        for (idx, item) in line.split_whitespace().enumerate() {
            if item.parse::<u64>().is_err() {
                problems[idx].operator = match item {
                    "*" => Operator::MUL,
                    "+" => Operator::ADD,
                    _ => panic!("Unrecognized operator parse!")
                };
                continue;
            }
        }
    }

    let mut column_ranges: Vec<(usize, usize)> = find_lexical_digit_ranges(input);
    
    let mut numbers: Vec<Vec<u64>> = Vec::new();

    let mut grid: Vec<Vec<char>> = input.split("\n")
                                        .map(|line| line.chars().collect())
                                        .collect();
    grid.pop();

    for range in column_ranges {
        numbers.push(Vec::new());

        for col in range.0..range.1+1 {
            let mut number: u64 = 0;
            for row in &grid {
                if row[col].is_numeric() {
                    number *= 10;
                    number += row[col].to_digit(10).unwrap() as u64;
                }
            }
            numbers.last_mut().unwrap().push(number);
        }
    }

    for (i, numbers) in numbers.iter().enumerate() {
        problems[i].numbers = numbers.to_vec();
    }

    problems
}

pub fn run_challenge_06_01(input_name: &str) -> u64 {
    let input = read_by_line(&("./src/challenges/challenge_06/".to_owned() + input_name));

    let problems = parse_problems_v2(&input);

    let mut sum: u64 = 0;

    for problem in problems {
        sum += solve_problem(&problem);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_01_sample() {
        assert_eq!(run_challenge_06_00("input_sample_a"), 4277556);
    }

    #[test]
    fn challenge_02_sample() {
        assert_eq!(run_challenge_06_01("input_sample_a"), 3263827);
    }
}

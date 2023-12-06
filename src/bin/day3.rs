use std::fs;

#[derive(Debug, PartialEq)]
enum EnginePartType {
    Number(String),
    Part(char),
}

#[derive(Debug)]
struct EnginePart {
    part_type: EnginePartType,
    row: usize,
    start_index: usize,
}

type Day3 = Vec<Vec<EnginePart>>;

fn main() {
    let input = read_input("inputs/day3.txt");

    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day3 {
    let input = fs::read_to_string(path).unwrap();

    let mut parts: Day3 = vec![];

    let mut number_buffer = String::new();
    let mut start_index: Option<usize> = None;

    input.lines().enumerate().for_each(|(row_index, l)| {
        let mut engine_row: Vec<EnginePart> = vec![];

        for (column_index, c) in l.chars().enumerate() {
            match c {
                c if c.is_digit(10) => {
                    if start_index.is_none() {
                        start_index = Some(column_index)
                    }

                    number_buffer.push(c)
                }
                _ => {
                    add_number_part(&mut engine_row, &mut number_buffer, row_index, start_index);
                    start_index = None;

                    match c {
                        '.' => continue,
                        _ => {
                            let part = EnginePart {
                                row: row_index,
                                start_index: column_index,
                                part_type: EnginePartType::Part(c),
                            };

                            engine_row.push(part);
                        }
                    }
                }
            }
        }

        let last_index = l.len() - number_buffer.len();

        add_number_part(
            &mut engine_row,
            &mut number_buffer,
            row_index,
            Some(last_index),
        );

        parts.push(engine_row);
    });

    parts
}

fn add_number_part(
    engine_row: &mut Vec<EnginePart>,
    number_buffer: &mut String,
    row_index: usize,
    column_index: Option<usize>,
) {
    if !number_buffer.is_empty() {
        let part = EnginePart {
            row: row_index,
            start_index: column_index.unwrap(),
            part_type: EnginePartType::Number(number_buffer.clone()),
        };

        engine_row.push(part);

        number_buffer.clear()
    }
}

fn puzzle1(input: &Day3) -> u32 {
    let mut real_parts: Vec<u32> = vec![];

    for row in input.iter() {
        for part in row.into_iter().filter(|p| match p.part_type {
            EnginePartType::Number(_) => true,
            EnginePartType::Part(_) => false,
        }) {
            match &part.part_type {
                EnginePartType::Number(number) => {
                    if check_if_adjacent(&input, part, &number) {
                        let part_number: u32 = number.parse::<u32>().unwrap();
                        real_parts.push(part_number)
                    }
                }
                EnginePartType::Part(_) => continue,
            }
        }
    }

    real_parts.iter().sum()
}

fn check_if_adjacent(
    input: &Day3,
    &EnginePart {
        part_type: _,
        row,
        start_index,
    }: &EnginePart,
    number: &String,
) -> bool {
    let min = if start_index == 0 { 0 } else { start_index - 1 };
    let max = start_index + number.len();

    // LEFT RIGHT
    if input[row].iter().any(|p| match p.part_type {
        EnginePartType::Number(_) => false,
        EnginePartType::Part(_) => (min..=max).contains(&p.start_index),
    }) {
        return true;
    }

    // TOP
    if row > 0 {
        if input[row - 1].iter().any(|p| match p.part_type {
            EnginePartType::Number(_) => false,
            EnginePartType::Part(_) => (min..=max).contains(&p.start_index),
        }) {
            return true;
        }
    }

    // BOTTOM
    if row < input.len() - 1 {
        if input[row + 1].iter().any(|p| match p.part_type {
            EnginePartType::Number(_) => false,
            EnginePartType::Part(_) => (min..=max).contains(&p.start_index),
        }) {
            return true;
        }
    }

    false
}

fn puzzle2(input: &Day3) -> u32 {
    let mut gears: Vec<u32> = vec![];

    for row in input.iter() {
        for part in row.into_iter().filter(|p| match p.part_type {
            EnginePartType::Number(_) => false,
            EnginePartType::Part('*') => true,
            EnginePartType::Part(_) => false,
        }) {
            if let Some(gear) = get_gear_ratio(&input, &part) {
                gears.push(gear)
            }
        }
    }

    gears.iter().sum()
}

fn get_gear_ratio(
    input: &Day3,
    &EnginePart {
        part_type: _,
        row,
        start_index,
    }: &EnginePart,
) -> Option<u32> {
    let mut numbers: Vec<String> = vec![];

    let min = if start_index == 0 { 0 } else { start_index - 1 };
    let max = start_index + 1;

    // LEFT RIGHT
    add_valid_numbers(&input[row], &mut numbers, min, max);

    // TOP
    if row > 0 {
        add_valid_numbers(&input[row - 1], &mut numbers, min, max);
    }

    // BOTTOM
    if row < input.len() - 1 {
        add_valid_numbers(&input[row + 1], &mut numbers, min, max);
    }

    if numbers.len() == 2 {
        let ratio: u32 = numbers
            .iter()
            .fold(1, |acc, n| acc * n.parse::<u32>().unwrap());

        return Some(ratio);
    }

    None
}

fn add_valid_numbers(row: &Vec<EnginePart>, numbers: &mut Vec<String>, min: usize, max: usize) {
    row.iter().for_each(|p| match &p.part_type {
        EnginePartType::Part(_) => (),
        EnginePartType::Number(number) => {
            let end_index = &p.start_index + number.len() - 1;
            if (min..=max).contains(&p.start_index) || (min..=max).contains(&end_index) {
                numbers.push(number.to_string());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/day3_test.txt";

    #[test]
    fn puzzle1_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle1(&test_input), 4361)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle2(&test_input), 467835);
    }
}

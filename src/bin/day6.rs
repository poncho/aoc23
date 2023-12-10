use std::{fs, str::Lines};

type Day6 = (Vec<u32>, Vec<u32>);

fn main() {
    let input = read_input("inputs/day6.txt");

    let p1_result = puzzle1(&input);
    println!("Puzzle #1: {}", p1_result);
}

fn read_input(path: &str) -> Day6 {
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();

    let race_times: Vec<u32> = parse_next_line(&mut lines);
    let record_times: Vec<u32> = parse_next_line(&mut lines);

    (race_times, record_times)
}

fn parse_next_line(lines: &mut Lines) -> Vec<u32> {
    let (_, values) = lines.next().unwrap().split_once(':').unwrap();

    values
        .split_ascii_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

fn puzzle1(input: &Day6) -> usize {
    let (race_times, record_times) = input;

    race_times
        .iter()
        .zip(record_times)
        .map(|(race_time, record_time)| {
            (0..=*race_time)
                .filter(|hold_time| hold_time * (race_time - hold_time) > *record_time)
                .count()
        })
        .reduce(|acc, wins| acc * wins)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/day6_test.txt";

    #[test]
    fn puzzle1_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle1(&test_input), 288)
    }
}

use std::{fs, str::Lines};

type Day6 = (Vec<u32>, Vec<u32>);

fn main() {
    let input = read_input("inputs/day6.txt");

    let p1_result = puzzle1(&input);
    println!("Puzzle #1: {}", p1_result);

    let p2_result = puzzle2(&input);
    println!("Puzzle #2: {}", p2_result);
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

fn puzzle2(input: &Day6) -> usize {
    let (race_times, record_times) = input;

    let race = fold_to_one_time(race_times);
    let record = fold_to_one_time(record_times);

    (0..=race)
        .filter(|hold_time| hold_time * (race - hold_time) > record)
        .count()
}

fn fold_to_one_time(times: &Vec<u32>) -> usize {
    times
        .iter()
        .map(|t| t.to_string())
        .reduce(|acc: String, t| acc + &t)
        .unwrap()
        .parse::<usize>()
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

    #[test]
    fn puzzle2_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle2(&test_input), 71503)
    }
}

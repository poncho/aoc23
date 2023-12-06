use regex::Regex;
use std::fs;

type Day4 = Vec<(Vec<String>, Vec<String>)>;

fn main() {
    let input = read_input("inputs/day4.txt");

    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day4 {
    let input = fs::read_to_string(path).unwrap();

    let re = Regex::new(r"Card\s+(?<card>\d+):(?<results>.*)").unwrap();

    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let (winner_str, card_str) = caps["results"].split_once('|').unwrap();

            let winners: Vec<String> = build_card(winner_str);
            let card: Vec<String> = build_card(card_str);

            (winners, card)
        })
        .collect()
}

fn build_card(card_str: &str) -> Vec<String> {
    card_str
        .trim()
        .split(' ')
        .map(|v| v.trim().to_string())
        .filter(|v| v.len() > 0)
        .collect()
}

fn puzzle1(input: &Day4) -> usize {
    const BASE: usize = 2;

    input
        .iter()
        .map(|(winners, card)| {
            let total_win_numbers: u32 =
                card.iter().filter(|c| winners.contains(&c)).count() as u32;

            let value = if total_win_numbers > 0 {
                BASE.pow(total_win_numbers - 1)
            } else {
                0
            };

            value
        })
        .sum()
}

fn puzzle2(input: &Day4) -> usize {
    let mut extra_list: Vec<usize> = vec![0; input.len()];

    input
        .iter()
        .map(|(winners, card)| card.iter().filter(|c| winners.contains(&c)).count() as usize)
        .enumerate()
        .for_each(|(i, wins)| {
            extra_list[i] += 1;
            let copies = extra_list[i];

            (i + 1..=i + wins).for_each(|extra_index| extra_list[extra_index] += copies)
        });

    extra_list.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/day4_test.txt";

    #[test]
    fn puzzle1_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle1(&test_input), 13);
    }

    #[test]
    fn puzzle2_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle2(&test_input), 30);
    }
}

use std::fs;

type Day1 = Vec<String>;

fn main() {
    let input = read_input("inputs/day1.txt");
    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day1 {
    let input = fs::read_to_string(path).unwrap();

    input.lines().map(|l| l.to_owned()).collect()
}

fn puzzle1(input: &Day1) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut first_digit: Option<u32> = None;
            let mut last_digit: u32 = 0;

            for c in line.chars() {
                if c.is_digit(10) {
                    let digit: u32 = c.to_digit(10).unwrap();

                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    }

                    last_digit = digit;
                }
            }

            (first_digit.unwrap() * 10) + last_digit
        })
        .sum()
}

fn puzzle2(input: &Day1) -> u32 {
    input
        .iter()
        .map(|line| {
            let mut first_digit: Option<u32> = None;
            let mut last_digit: u32 = 0;
            let mut letter_digit: String = String::new();

            for c in line.chars() {
                let mut digit: Option<u32> = None;

                if c.is_digit(10) {
                    digit = Some(c.to_digit(10).unwrap());
                } else {
                    letter_digit.push(c);
                    digit = read_letter_digit(&letter_digit);
                }

                if let Some(d) = digit {
                    if first_digit.is_none() {
                        first_digit = digit;
                    }

                    last_digit = d;
                }
            }

            (first_digit.unwrap() * 10) + last_digit;
        })
        .sum()
}

fn read_letter_digit(letter_digit: &str) -> Option<u32> {
    match letter_digit {
        _ if letter_digit.ends_with("one") => Some(1),
        _ if letter_digit.ends_with("two") => Some(2),
        _ if letter_digit.ends_with("three") => Some(3),
        _ if letter_digit.ends_with("four") => Some(4),
        _ if letter_digit.ends_with("five") => Some(5),
        _ if letter_digit.ends_with("six") => Some(6),
        _ if letter_digit.ends_with("seven") => Some(7),
        _ if letter_digit.ends_with("eight") => Some(8),
        _ if letter_digit.ends_with("nine") => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let test_input = read_input("inputs/day1_1_test.txt");

        assert_eq!(puzzle1(&test_input), 142)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = read_input("inputs/day1_2_test.txt");
        assert_eq!(puzzle2(&test_input), 281)
    }
}

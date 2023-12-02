use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

type Day2 = Vec<Game>;

fn main() {
    let input = read_input("inputs/day2.txt");

    let p1_result = puzzle1(&input);
    let p2_result = puzzle2(&input);

    println!("Puzzle #1: {}", p1_result);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> Day2 {
    let input = fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|l| {
            let mut parts = l.split(":");
            let game_data = parts.next().unwrap();
            let id: u32;

            id = game_data
                .split(" ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let rounds: Vec<Round> = parts
                .next()
                .unwrap()
                .split(";")
                .map(|s| parse_round(s))
                .collect();

            Game { id, rounds }
        })
        .collect()
}

fn parse_round(round_data: &str) -> Round {
    let mut round = Round {
        green: 0,
        blue: 0,
        red: 0,
    };

    for pick in round_data.trim().split(",") {
        let mut pick_data = pick.trim().split(" ");
        let total_cubes: u32 = pick_data.next().unwrap().parse::<u32>().unwrap();

        match pick_data.next().unwrap() {
            "blue" => round.blue += total_cubes,
            "green" => round.green += total_cubes,
            _ => round.red += total_cubes,
        }
    }

    round
}

fn puzzle1(input: &Day2) -> u32 {
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    input
        .iter()
        .filter(|game| {
            let mut valid = true;

            for Round { red, green, blue } in game.rounds.iter() {
                if *red > max_red || *green > max_green || *blue > max_blue {
                    valid = false;
                    break;
                }
            }

            valid
        })
        .map(|game| game.id)
        .sum::<u32>()
}

fn puzzle2(input: &Day2) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for Round { red, green, blue } in game.rounds.iter() {
                if *red > min_red {
                    min_red = *red
                }
                if *green > min_green {
                    min_green = *green
                }
                if *blue > min_blue {
                    min_blue = *blue
                }
            }

            min_red * min_green * min_blue
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let test_input = read_input("inputs/day2_test.txt");

        assert_eq!(puzzle1(&test_input), 8)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = read_input("inputs/day2_test.txt");

        assert_eq!(puzzle2(&test_input), 2286)
    }
}

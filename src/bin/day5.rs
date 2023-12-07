use std::{collections::HashMap, fs};

type AlmanacMap = Vec<(usize, usize, usize)>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum MapperType {
    SeedSoil,
    SoilFertilizer,
    FertilizerWater,
    WaterLight,
    LightTemperature,
    TemperatureHumidity,
    HumidityLocation,
}

#[derive(Debug)]
struct SeedMapper {
    seeds: Vec<usize>,
    mappers: HashMap<MapperType, AlmanacMap>,
}

fn main() {
    let input = read_input("inputs/day5.txt");

    let p1_result = puzzle1(&input);
    println!("Puzzle #1: {}", p1_result);

    let p2_result = puzzle2(&input);
    println!("Puzzle #2: {}", p2_result);
}

fn read_input(path: &str) -> SeedMapper {
    let input = fs::read_to_string(path).unwrap();

    let mut seed_mapper = SeedMapper {
        seeds: vec![],
        mappers: HashMap::from([
            (MapperType::SeedSoil, vec![]),
            (MapperType::SoilFertilizer, vec![]),
            (MapperType::FertilizerWater, vec![]),
            (MapperType::WaterLight, vec![]),
            (MapperType::LightTemperature, vec![]),
            (MapperType::TemperatureHumidity, vec![]),
            (MapperType::HumidityLocation, vec![]),
        ]),
    };

    let mut current_mapper: MapperType = MapperType::SeedSoil;

    input
        .lines()
        .filter(|l| !l.is_empty())
        .for_each(|line| match line {
            "seed-to-soil map:" => current_mapper = MapperType::SeedSoil,
            "soil-to-fertilizer map:" => current_mapper = MapperType::SoilFertilizer,
            "fertilizer-to-water map:" => current_mapper = MapperType::FertilizerWater,
            "water-to-light map:" => current_mapper = MapperType::WaterLight,
            "light-to-temperature map:" => current_mapper = MapperType::LightTemperature,
            "temperature-to-humidity map:" => current_mapper = MapperType::TemperatureHumidity,
            "humidity-to-location map:" => current_mapper = MapperType::HumidityLocation,
            _ if line.starts_with("seeds") => {
                seed_mapper.seeds = line
                    .split_ascii_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
            }
            _ => {
                let values: Vec<usize> = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                let dest = values[0];
                let source = values[1];
                let range = values[2];

                seed_mapper
                    .mappers
                    .entry(current_mapper.clone())
                    .and_modify(|ranges| ranges.push((source, dest, range)));
            }
        });

    seed_mapper
}

fn puzzle1(input: &SeedMapper) -> usize {
    input
        .seeds
        .iter()
        .map(|seed| get_seed_location(input, seed))
        .min()
        .unwrap()
}

fn puzzle2(input: &SeedMapper) -> usize {
    input
        .seeds
        .chunks(2)
        .map(|chunk| {
            let range_start = chunk[0];
            let length = chunk[1];

            (range_start..range_start + length)
                .map(|seed| get_seed_location(input, &seed))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn get_seed_location(input: &SeedMapper, seed: &usize) -> usize {
    let soil: usize = next_step(input, &MapperType::SeedSoil, seed);
    let fertilizer: usize = next_step(input, &MapperType::SoilFertilizer, &soil);
    let water: usize = next_step(input, &MapperType::FertilizerWater, &fertilizer);
    let light: usize = next_step(input, &MapperType::WaterLight, &water);
    let temperature: usize = next_step(input, &MapperType::LightTemperature, &light);
    let humidity: usize = next_step(input, &MapperType::TemperatureHumidity, &temperature);

    next_step(input, &MapperType::HumidityLocation, &humidity)
}

fn next_step(input: &SeedMapper, mapper: &MapperType, value: &usize) -> usize {
    match input
        .mappers
        .get(mapper)
        .unwrap()
        .iter()
        .find(|(source, _dest, range_length)| *value >= *source && *value < *source + *range_length)
    {
        Some((source, dest, _range_length)) => dest + (value - source),
        None => *value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "inputs/day5_test.txt";

    #[test]
    fn puzzle1_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle1(&test_input), 35)
    }

    #[test]
    fn puzzle2_test() {
        let test_input = read_input(TEST_FILE);

        assert_eq!(puzzle2(&test_input), 46)
    }
}

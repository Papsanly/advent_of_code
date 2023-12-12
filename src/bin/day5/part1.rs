use crate::map::Map;

pub fn part1(input: &str) -> Option<usize> {
    let seeds = input
        .lines()
        .next()?
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().ok())
        .collect::<Option<Vec<usize>>>()?;

    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|map| map.parse().ok())
        .collect::<Option<Vec<Map>>>()?;

    let locations = seeds.into_iter().map(|seed| {
        maps.iter()
            .fold(seed, |acc, map| map.get(acc).unwrap_or(acc))
    });

    locations.min()
}

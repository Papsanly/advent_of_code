use crate::parse;

pub fn part1(input: &str) -> Option<usize> {
    let seeds = parse::seeds(input)?;

    let maps = parse::maps(input)?;

    let locations = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, map| map.get(acc)));

    locations.min()
}

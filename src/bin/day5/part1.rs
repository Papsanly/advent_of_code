use std::ops::{Deref, Range};
use std::str::FromStr;

struct MapRange {
    destination_range: Range<usize>,
    source_range: Range<usize>,
}

impl MapRange {
    fn contains(&self, value: usize) -> bool {
        self.source_range.contains(&value)
    }

    fn get(&self, value: usize) -> usize {
        self.destination_range.start + value - self.source_range.start
    }
}

impl FromStr for MapRange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [destination_start, source_start, range_length] = s
            .split_whitespace()
            .map(|v| v.parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| ())?;
        Ok(Self {
            destination_range: destination_start..destination_start + range_length,
            source_range: source_start..source_start + range_length,
        })
    }
}

struct Map(Vec<MapRange>);

impl Deref for Map {
    type Target = Vec<MapRange>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Map {
    fn get(&self, key: usize) -> Option<usize> {
        for range in self.iter() {
            if range.contains(key) {
                return Some(range.get(key));
            }
        }
        None
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        ))
    }
}

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

use std::ops::Deref;
use std::str::FromStr;

struct Range {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl Range {
    fn contains(&self, value: usize) -> bool {
        value >= self.source_start && value <= self.source_start + self.range_length
    }

    fn get(&self, value: usize) -> usize {
        self.destination_start + value - self.source_start
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [destination_start, source_start, range_length] = s
            .split_whitespace()
            .map(|v| v.parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| ())?;
        Ok(Self {
            destination_start,
            source_start,
            range_length,
        })
    }
}

struct Map(Vec<Range>);

impl Deref for Map {
    type Target = Vec<Range>;
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

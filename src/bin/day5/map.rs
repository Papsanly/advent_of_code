use std::ops::{Deref, Range};
use std::str::FromStr;

pub struct MapRange {
    destination: Range<usize>,
    source: Range<usize>,
}

impl MapRange {
    pub fn contains(&self, value: usize) -> bool {
        self.source.contains(&value)
    }

    pub fn get(&self, value: usize) -> usize {
        self.destination.start + value - self.source.start
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
            destination: destination_start..destination_start + range_length,
            source: source_start..source_start + range_length,
        })
    }
}

pub struct Map(Vec<MapRange>);

impl Deref for Map {
    type Target = Vec<MapRange>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Map {
    pub fn get(&self, key: usize) -> Option<usize> {
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

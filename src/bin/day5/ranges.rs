use std::ops;
use std::str::FromStr;

pub type Range = ops::Range<usize>;

#[derive(Debug)]
pub struct Map {
    source: Vec<Range>,
    destination: Vec<Range>,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut source = Vec::new();
        let mut destination = Vec::new();

        for line in s.lines().skip(1) {
            let [destination_start, source_start, range_length] = line
                .split_whitespace()
                .map(|v| v.parse().map_err(|_| ()))
                .collect::<Result<Vec<usize>, _>>()?
                .try_into()
                .map_err(|_| ())?;
            source.push(source_start..source_start + range_length);
            destination.push(destination_start..destination_start + range_length);
        }

        Ok(Self {
            source,
            destination,
        })
    }
}

impl Map {
    pub fn get(&self, key: usize) -> usize {
        for (source, destination) in self.source.iter().zip(self.destination.iter()) {
            if source.contains(&key) {
                return destination.start + key - source.start;
            }
        }
        key
    }

    pub fn get_range(&self, key_range: &Range) -> Vec<Range> {
        todo!()
    }
}

use crate::ranges::Map;

pub fn seeds(input: &str) -> Option<Vec<usize>> {
    input
        .lines()
        .next()?
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().ok())
        .collect::<Option<Vec<usize>>>()
}

pub fn maps(input: &str) -> Option<Vec<Map>> {
    input
        .split("\n\n")
        .skip(1)
        .map(|map| map.parse().ok())
        .collect::<Option<Vec<Map>>>()
}

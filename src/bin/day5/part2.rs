use crate::parse;
use crate::ranges::Range;

pub fn part2(input: &str) -> Option<usize> {
    let seeds = parse::seeds(input)?
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<Vec<Range>>();

    let maps = parse::maps(input)?;

    let mut result = seeds;
    for map in maps {
        let mut new_result = Vec::new();
        for range in &result {
            new_result.append(&mut map.get_range(range))
        }
        result = new_result;
    }

    result.iter().map(|range| range.start).min()
}

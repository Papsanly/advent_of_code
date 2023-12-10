use crate::gear::gear_iterator;
use crate::part_number::PartNumber;
use crate::schematic::Schematic;
use std::collections::HashSet;
use std::hash::Hash;

fn unique_values<I, T>(iter: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
    T: Eq + Hash + Copy,
{
    let mut seen = HashSet::new();
    iter.filter(move |x| seen.insert(*x))
}

pub fn part2(input: &str) -> Option<usize> {
    let schematic = Schematic::new(input);

    let values: Vec<Option<usize>> = gear_iterator(&schematic)
        .filter_map(|g| {
            let adjacent_digits: Vec<PartNumber> = unique_values(
                schematic
                    .get_adjacent(&g)
                    .into_iter()
                    .filter_map(|indices| PartNumber::from_idx(&schematic, indices)),
            )
            .collect();

            if adjacent_digits.len() != 2 {
                None
            } else {
                Some(
                    adjacent_digits
                        .into_iter()
                        .map(|pn| pn.value)
                        .reduce(|acc, pn| acc * pn),
                )
            }
        })
        .collect();

    values.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        assert_eq!(
            part2(
                "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            Some(467835)
        )
    }
}

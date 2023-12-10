use crate::gear::gear_iterator;
use crate::part_number::PartNumber;
use crate::schematic::Schematic;

pub fn part2(input: &str) -> Option<usize> {
    let schematic = Schematic::new(input);

    let values: Vec<Option<usize>> = gear_iterator(&schematic)
        .filter_map(|g| {
            let adjacent_digits: Vec<usize> = [
                schematic.get_top_adjacent(&g),
                schematic.get_bottom_adjacent(&g),
                schematic.get_left_adjacent(&g),
                schematic.get_right_adjacent(&g),
            ]
            .into_iter()
            .filter_map(|indices| {
                Some(
                    PartNumber::from_idx(
                        &schematic,
                        indices
                            .into_iter()
                            .find(|&i| schematic[i].is_ascii_digit())?,
                    )
                    .unwrap()
                    .value,
                )
            })
            .collect();

            dbg!(&adjacent_digits);

            if adjacent_digits.len() != 2 {
                None
            } else {
                Some(adjacent_digits.into_iter().reduce(|acc, val| acc * val))
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

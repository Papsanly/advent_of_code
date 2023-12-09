use crate::part_number::PartNumberIterator;
use crate::schematic::Schematic;
use crate::INPUT;

pub fn part1() -> Option<usize> {
    let schematic = Schematic::new(INPUT);

    let values: Vec<usize> = PartNumberIterator::new(&schematic)
        .filter(|pn| {
            schematic
                .get_adjacent(pn)
                .iter()
                .any(|&c| !c.is_ascii_digit() && c != '.')
        })
        .map(|pn| pn.value)
        .collect();

    dbg!(&values);

    Some(values.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::part_number::PartNumberIterator;
    use crate::schematic::Schematic;

    #[test]
    fn part_1() {
        let schematic = Schematic::new(
            "
.........232.633.......................803....
.............*........361...............-.....
..........539..............2..973.221...340...
...329..........*..............#.....256.#....
......................*313............*.......
...766.......*......72...*...-...........+.249
6*.-..@.......181........4..865.........968..6
",
        );
        let mut values: Vec<usize> = Vec::new();
        for num in PartNumberIterator::new(&schematic) {
            if schematic
                .get_adjacent(&num)
                .iter()
                .any(|&c| !c.is_ascii_digit() && c != '.')
            {
                values.push(num.value);
            }
        }

        assert_eq!(
            values,
            vec![633, 803, 539, 973, 340, 256, 313, 766, 72, 6, 181, 4, 865, 968]
        )
    }
}

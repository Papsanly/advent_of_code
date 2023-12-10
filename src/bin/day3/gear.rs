use crate::schematic::{AdjacentIndices, Schematic, SchematicIndex};

pub struct Gear {
    pub index: SchematicIndex,
}

impl AdjacentIndices for Gear {
    fn index(&self) -> SchematicIndex {
        self.index
    }

    fn span(&self) -> usize {
        1
    }
}

impl Gear {
    pub fn new(schematic: &Schematic, index: usize) -> Option<Self> {
        let char = schematic.chars[index];
        if char != '*' {
            None
        } else {
            Some(Self {
                index: SchematicIndex::new(schematic, index),
            })
        }
    }
}

pub fn gear_iterator(schematic: &Schematic) -> impl Iterator<Item = Gear> + '_ {
    schematic.chars.iter().enumerate().filter_map(|(i, &c)| {
        if c == '*' {
            Gear::new(schematic, i)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::Schematic;

    #[test]
    fn part_number_from_idx() {
        let schematic = Schematic::new(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
.......755
...$.*....
.664.598.6",
        );

        assert_eq!(
            gear_iterator(&schematic)
                .map(|g| (g.index.x, g.index.y))
                .collect::<Vec<_>>(),
            vec![(3, 1), (3, 4), (5, 8)]
        )
    }
}

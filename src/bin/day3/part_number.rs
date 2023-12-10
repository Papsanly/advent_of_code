use crate::schematic::{AdjacentIndices, Schematic, SchematicIndex};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PartNumber {
    pub value: usize,
    pub span: usize,
    pub index: SchematicIndex,
}

impl PartNumber {
    pub fn new(schematic: &Schematic, start: usize, end: usize) -> Option<Self> {
        Some(Self {
            value: schematic.chars[start..end]
                .iter()
                .collect::<String>()
                .parse()
                .ok()?,
            span: end - start,
            index: SchematicIndex::new(schematic, start),
        })
    }

    pub fn from_idx(schematic: &Schematic, index: SchematicIndex) -> Option<Self> {
        let index = index.to_pos(schematic);

        if !schematic.chars.get(index)?.is_ascii_digit() {
            return None;
        }

        let start = schematic.chars[..index]
            .iter()
            .rposition(|&c| !c.is_ascii_digit())
            .map_or(0, |start| start + 1);

        let end = schematic.chars[index..]
            .iter()
            .position(|&c| !c.is_ascii_digit())
            .map_or(schematic.chars.len(), |end| end + index);

        PartNumber::new(schematic, start, end)
    }
}

impl AdjacentIndices for PartNumber {
    fn span(&self) -> usize {
        self.span
    }

    fn index(&self) -> SchematicIndex {
        self.index
    }
}

pub struct PartNumberIterator<'a> {
    schematic: &'a Schematic,
    pos: usize,
}

impl<'a> PartNumberIterator<'a> {
    pub fn new(s: &'a Schematic) -> Self {
        Self {
            schematic: s,
            pos: 0,
        }
    }
}

impl<'a> Iterator for PartNumberIterator<'a> {
    type Item = PartNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.schematic.chars[self.pos..]
            .iter()
            .position(|c| c.is_ascii_digit())?
            + self.pos;

        let end = self.schematic.chars[start..]
            .iter()
            .position(|c| !c.is_ascii_digit())
            .map_or(self.schematic.chars.len(), |end| end + start);

        self.pos = end;

        PartNumber::new(self.schematic, start, end)
    }
}

#[cfg(test)]
mod tests {
    use crate::part_number::{PartNumber, PartNumberIterator};
    use crate::schematic::Schematic;

    #[test]
    fn part_number_iterator() {
        let schematic = Schematic::new(
            "
...329...*.....#.....256.#....
.&....*313............*.......
...766....*......72...-..+.249
6..-..@....181..4..865..968..6
",
        );
        assert_eq!(
            PartNumberIterator::new(&schematic)
                .map(|pn| (pn.value, pn.index, pn.span))
                .collect::<Vec<_>>(),
            vec![
                (329, (3, 0).into(), 3),
                (256, (21, 0).into(), 3),
                (313, (7, 1).into(), 3),
                (766, (3, 2).into(), 3),
                (72, (17, 2).into(), 2),
                (249, (27, 2).into(), 3),
                (6, (0, 3).into(), 1),
                (181, (11, 3).into(), 3),
                (4, (16, 3).into(), 1),
                (865, (19, 3).into(), 3),
                (968, (24, 3).into(), 3),
                (6, (29, 3).into(), 1)
            ]
        )
    }

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
            PartNumber::from_idx(&schematic, (0, 0).into())
                .unwrap()
                .value,
            467
        );
        assert_eq!(
            PartNumber::from_idx(&schematic, (1, 0).into())
                .unwrap()
                .value,
            467
        );
        assert_eq!(
            PartNumber::from_idx(&schematic, (2, 0).into())
                .unwrap()
                .value,
            467
        );
        assert_eq!(PartNumber::from_idx(&schematic, (3, 0).into()), None);
        assert_eq!(
            PartNumber::from_idx(&schematic, (2, 6).into())
                .unwrap()
                .value,
            592
        );
        assert_eq!(
            PartNumber::from_idx(&schematic, (8, 7).into())
                .unwrap()
                .value,
            755
        );
        assert_eq!(
            PartNumber::from_idx(&schematic, (9, 9).into())
                .unwrap()
                .value,
            6
        );
        assert_eq!(PartNumber::from_idx(&schematic, (11, 11).into()), None);
        assert_eq!(PartNumber::from_idx(&schematic, (4, 9).into()), None);
    }
}

use crate::schematic::{Schematic, SchematicIndex};

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
        todo!()
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
    use crate::part_number::PartNumberIterator;
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
}

use crate::INPUT;
use std::ops::{Index, Range};

type SchematicIndex = (usize, usize);

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
}

struct Schematic {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl Schematic {
    fn new(s: &str) -> Self {
        let chars: Vec<char> = s.trim().chars().collect();
        let width = s.trim().find('\n').unwrap_or(s.len());
        let height = chars.len() / width;
        Self {
            chars,
            width,
            height,
        }
    }

    fn move_idx(&self, index: SchematicIndex, direction: Direction) -> Option<SchematicIndex> {
        use Direction::*;
        let (mut x, mut y) = index;

        if let Up | UpLeft | UpRight = direction {
            y = y.checked_sub(1)?;
        } else if let Down | DownLeft | DownRight = direction {
            y += 1;
            if y > self.height {
                return None;
            }
        }

        if let Left | UpLeft | DownLeft = direction {
            x = x.checked_sub(1)?;
        } else if let Right | UpRight | DownRight = direction {
            x += 1;
            if x > self.width {
                return None;
            }
        }

        Some((x, y))
    }

    fn get_adjacent(&self, pn: &PartNumber) -> Vec<char> {
        todo!()
    }
}

impl Index<SchematicIndex> for Schematic {
    type Output = char;
    fn index(&self, index: SchematicIndex) -> &Self::Output {
        &self.chars[index.1 * (self.width + 1) + index.0]
    }
}

struct PartNumber {
    value: usize,
    span: usize,
    index: SchematicIndex,
}

impl PartNumber {
    fn new(schematic: &Schematic, start: usize, end: usize) -> Option<Self> {
        Some(Self {
            value: schematic.chars[start..end]
                .iter()
                .collect::<String>()
                .parse()
                .ok()?,
            span: end - start,
            index: (start % (schematic.width + 1), start / (schematic.width + 1)),
        })
    }
}

struct PartNumberIterator<'a> {
    schematic: &'a Schematic,
    pos: usize,
}

impl<'a> PartNumberIterator<'a> {
    fn new(s: &'a Schematic) -> Self {
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

pub fn part1() -> Option<usize> {
    let schematic = Schematic::new(INPUT);

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

    dbg!(&values);

    Some(values.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_indices() {
        use Direction::*;

        let schematic = Schematic::new(INPUT);
        for direction in [Up, Left, UpLeft, DownLeft, UpRight] {
            assert_eq!(schematic.move_idx((0, 0), direction), None);
        }

        assert_eq!(schematic.move_idx((0, 0), Right), Some((1, 0)));
        assert_eq!(schematic.move_idx((0, 0), Down), Some((0, 1)));
        assert_eq!(schematic.move_idx((0, 0), DownRight), Some((1, 1)));

        for direction in [Down, Right, DownRight, DownLeft, UpRight] {
            assert_eq!(
                schematic.move_idx((schematic.width, schematic.height), direction),
                None
            );
        }

        assert_eq!(
            schematic.move_idx((schematic.width, schematic.height), Up),
            Some((schematic.width, schematic.height - 1))
        );
        assert_eq!(
            schematic.move_idx((schematic.width, schematic.height), Left),
            Some((schematic.width - 1, schematic.height))
        );
        assert_eq!(
            schematic.move_idx((schematic.width, schematic.height), UpLeft),
            Some((schematic.width - 1, schematic.height - 1))
        );
    }

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
                (329, (3, 0), 3),
                (256, (21, 0), 3),
                (313, (7, 1), 3),
                (766, (3, 2), 3),
                (72, (17, 2), 2),
                (249, (27, 2), 3),
                (6, (0, 3), 1),
                (181, (11, 3), 3),
                (4, (16, 3), 1),
                (865, (19, 3), 3),
                (968, (24, 3), 3),
                (6, (29, 3), 1)
            ]
        )
    }

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

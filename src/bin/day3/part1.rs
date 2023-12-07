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
        let chars: Vec<char> = s.chars().collect();
        let width = s.find('\n').map_or(s.len(), |w| w + 1);
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

    fn get_adjacent(&self, part_number: &PartNumber) -> Vec<char> {
        todo!()
    }
}

impl Index<SchematicIndex> for Schematic {
    type Output = char;
    fn index(&self, index: SchematicIndex) -> &Self::Output {
        &self.chars[index.1 * self.width + index.0]
    }
}

struct PartNumber {
    value: usize,
    span: Range<usize>,
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
            .unwrap_or(self.pos)
            + start;

        self.pos = end;

        let value = self.schematic.chars[start..end]
            .iter()
            .collect::<String>()
            .parse()
            .ok()?;

        Some(PartNumber {
            value,
            span: start..end,
        })
    }
}

pub fn part1() -> Option<usize> {
    let schematic = Schematic::new(INPUT);

    let mut values: Vec<usize> = Vec::new();
    for num in PartNumberIterator::new(&schematic) {
        // if num
        //     .get_adjacent()
        //     .iter()
        //     .any(|&c| !c.is_ascii_digit() && c != '.')
        // {
        values.push(num.value);
        // }
    }

    dbg!(&values);
    dbg!(schematic[(schematic.width, schematic.height)]);

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
.........232.633.......................803....
.............*........361...............-.....
..........539.................973.221...340...
...329..........*..............#.....256.#....
......................*313............*.......
...766.......*.....472.......-...........+.249
670-..@.......181......814..865.........968...
",
        );
        assert_eq!(
            PartNumberIterator::new(&schematic)
                .map(|pn| pn.value)
                .collect::<Vec<_>>(),
            vec![
                232, 633, 803, 361, 539, 973, 221, 340, 329, 256, 313, 766, 472, 249, 670, 181,
                814, 865, 968
            ]
        )
    }
}

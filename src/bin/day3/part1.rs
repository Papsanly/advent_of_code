use crate::INPUT;
use std::ops::Index;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct SchematicIndex {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for SchematicIndex {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

macro_rules! add_idx {
    ($schematic:expr, $first_idx:expr, $($idx:expr),*) => {'add: {
        let mut res = $first_idx;
        $(res = match $schematic.add_idx(res, $idx) {
            Some(res) => res,
            None => break 'add None
        };)*
        Some(res)
    }};
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

    fn add_idx<T>(&self, first: SchematicIndex, second: (T, T)) -> Option<SchematicIndex>
    where
        T: TryInto<isize>,
    {
        let x = first.x as isize + second.0.try_into().ok()?;
        let y = first.y as isize + second.1.try_into().ok()?;

        if x >= self.width as isize {
            return None;
        }
        if y >= self.height as isize {
            return None;
        }

        if x < 0 || y < 0 {
            None
        } else {
            Some((x as usize, y as usize).into())
        }
    }

    fn get_adjacent(&self, pn: &PartNumber) -> Vec<char> {
        let mut res = Vec::new();

        for offset in 0..pn.span {
            for direction in [(0, 1), (0, -1)] {
                if let Some(i) = add_idx!(self, pn.index, (offset, 0), direction) {
                    res.push(self[i])
                }
            }
        }

        for direction in [(-1, -1), (-1, 0), (-1, 1)] {
            if let Some(i) = add_idx!(self, pn.index, direction) {
                res.push(self[i])
            }
        }

        let offset_idx = add_idx!(self, pn.index, (pn.span - 1, 0)).unwrap();
        for direction in [(1, -1), (1, 0), (1, 1)] {
            if let Some(i) = add_idx!(self, offset_idx, direction) {
                res.push(self[i])
            }
        }

        res
    }
}

impl Index<SchematicIndex> for Schematic {
    type Output = char;
    fn index(&self, index: SchematicIndex) -> &Self::Output {
        &self.chars[index.y * (self.width + 1) + index.x]
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
            index: (start % (schematic.width + 1), start / (schematic.width + 1)).into(),
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
    use super::*;

    #[test]
    fn add_idx() {
        let schematic = Schematic::new(INPUT);
        assert_eq!(
            schematic.add_idx((schematic.width - 1, schematic.height - 1).into(), (1, 0)),
            None
        );
        assert_eq!(schematic.add_idx((0, 0).into(), (0, -1)), None);
        assert_eq!(
            schematic.add_idx((4, 5).into(), (4, -5)),
            Some((8, 0).into())
        );
        assert_eq!(
            add_idx!(schematic, (3, 5).into(), (-1, 2), (3, 4), (-2, -2)),
            Some((3, 9).into())
        )
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

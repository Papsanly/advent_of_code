use crate::INPUT;
use std::ops::Range;

struct Schematic {
    chars: Vec<char>,
    width: usize,
}

impl Schematic {
    fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            width: s.find('\n').map_or(s.len(), |w| w + 1),
        }
    }
}

struct PartNumber<'a> {
    schematic: &'a Schematic,
    span: Range<usize>,
}

impl<'a> PartNumber<'a> {
    fn get_adjacent(&self) -> Vec<char> {
        todo!()
    }

    fn value(&self) -> usize {
        self.schematic.chars[self.span.clone()]
            .iter()
            .collect::<String>()
            .parse()
            .ok()
            .unwrap()
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
    type Item = PartNumber<'a>;

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

        Some(PartNumber {
            schematic: self.schematic,
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
        values.push(num.value());
        // }
    }

    dbg!(&values);

    Some(values.into_iter().sum())
}

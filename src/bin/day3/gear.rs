use crate::schematic::{Schematic, SchematicIndex};

pub struct Gear {
    pub index: SchematicIndex,
}

impl Gear {
    pub fn new(schematic: &Schematic, index: SchematicIndex) -> Option<Self> {
        let char = schematic[index];
        if char != '*' {
            None
        } else {
            Some(Self { index })
        }
    }
}

pub struct GearIterator<'a> {
    schematic: &'a Schematic,
    pos: usize,
}

impl<'a> GearIterator<'a> {
    pub fn new(schematic: &'a Schematic) -> Self {
        Self { schematic, pos: 0 }
    }
}

impl<'a> Iterator for GearIterator<'a> {
    type Item = Gear;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.schematic.chars[self.pos..]
            .iter()
            .position(|&c| c == '*')?;
        self.pos = pos;
        Gear::new(self.schematic, SchematicIndex::new(self.schematic, pos))
    }
}

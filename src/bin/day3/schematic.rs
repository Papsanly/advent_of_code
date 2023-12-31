use std::ops::Index;

#[macro_export]
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

pub trait AdjacentIndices {
    fn span(&self) -> usize;
    fn index(&self) -> SchematicIndex;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct SchematicIndex {
    pub x: usize,
    pub y: usize,
}

impl SchematicIndex {
    pub fn new(schematic: &Schematic, index: usize) -> Self {
        (index % (schematic.width + 1), index / (schematic.width + 1)).into()
    }

    pub fn to_pos(self, schematic: &Schematic) -> usize {
        self.y * (schematic.width + 1) + self.x
    }
}

impl From<(usize, usize)> for SchematicIndex {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

pub struct Schematic {
    pub chars: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Schematic {
    pub fn new(s: &str) -> Self {
        let chars: Vec<char> = s.trim().chars().collect();
        let width = s.trim().find('\n').unwrap_or(s.len());
        let height = chars.len() / width;
        Self {
            chars,
            width,
            height,
        }
    }

    pub fn add_idx<T>(&self, first: SchematicIndex, second: (T, T)) -> Option<SchematicIndex>
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

    pub fn get_adjacent(&self, ai: &impl AdjacentIndices) -> Vec<SchematicIndex> {
        let mut res = Vec::new();

        for offset in 0..ai.span() {
            for direction in [(0, 1), (0, -1)] {
                if let Some(i) = add_idx!(self, ai.index(), (offset, 0), direction) {
                    res.push(i)
                }
            }
        }

        for direction in [(-1, -1), (-1, 0), (-1, 1)] {
            if let Some(i) = add_idx!(self, ai.index(), direction) {
                res.push(i)
            }
        }

        let offset_idx = add_idx!(self, ai.index(), (ai.span() - 1, 0)).unwrap();
        for direction in [(1, -1), (1, 0), (1, 1)] {
            if let Some(i) = add_idx!(self, offset_idx, direction) {
                res.push(i)
            }
        }

        res
    }
}

impl<T: Into<SchematicIndex>> Index<T> for Schematic {
    type Output = char;
    fn index(&self, index: T) -> &Self::Output {
        &self.chars[index.into().to_pos(self)]
    }
}

#[cfg(test)]
mod tests {
    use crate::schematic::Schematic;
    use crate::INPUT;

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
}

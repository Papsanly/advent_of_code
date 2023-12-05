use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub sets: Vec<Set>,
}

#[derive(Debug)]
pub struct Set {
    pub red: Option<usize>,
    pub green: Option<usize>,
    pub blue: Option<usize>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s.split_once(':').ok_or(())?;
        let id = id.split_once(' ').ok_or(())?.1.parse().map_err(|_| ())?;
        let sets: Vec<Set> = sets
            .split(';')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ()>>()?;
        Ok(Self { id, sets })
    }
}

impl FromStr for Set {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = None;
        let mut green = None;
        let mut blue = None;

        for color_value in s.split(',').map(str::trim) {
            let (value, color) = color_value.split_once(' ').ok_or(())?;
            let value = value.parse().ok();
            match color {
                "red" => red = value,
                "green" => green = value,
                "blue" => blue = value,
                _ => (),
            }
        }

        Ok(Set { red, green, blue })
    }
}

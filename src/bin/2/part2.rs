use crate::game::Game;
use crate::INPUT;

macro_rules! max_by_color {
    ($game:ident, $color:ident) => {
        $game
            .sets
            .iter()
            .max_by_key(|set| set.$color.unwrap_or(0))?
            .$color?
    };
}

pub fn part2() -> Option<usize> {
    let games = INPUT
        .lines()
        .map(str::parse)
        .map(Result::ok)
        .collect::<Option<Vec<Game>>>()?;

    games
        .iter()
        .map(|game| {
            Some(max_by_color!(game, red) * max_by_color!(game, green) * max_by_color!(game, blue))
        })
        .sum()
}

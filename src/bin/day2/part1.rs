use crate::game::Game;
use crate::INPUT;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

pub fn part1() -> Option<usize> {
    let games = INPUT
        .lines()
        .map(str::parse)
        .map(Result::ok)
        .collect::<Option<Vec<Game>>>()?;

    Some(
        games
            .iter()
            .filter(|game| {
                !game.sets.iter().any(|set| {
                    set.red.unwrap_or(0) > MAX_RED
                        || set.green.unwrap_or(0) > MAX_GREEN
                        || set.blue.unwrap_or(0) > MAX_BLUE
                })
            })
            .map(|game| game.id)
            .sum::<usize>(),
    )
}

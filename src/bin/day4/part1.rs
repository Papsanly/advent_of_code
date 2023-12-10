use crate::card::Card;

pub fn part1(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input
        .lines()
        .map(str::parse)
        .map(Result::ok)
        .collect::<Option<_>>()?;

    let points = cards.iter().map(|card| {
        let winnings = card.get_winning_count();
        if winnings != 0 {
            2usize.pow(winnings as u32 - 1)
        } else {
            0
        }
    });

    Some(points.sum())
}

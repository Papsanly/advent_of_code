use crate::card::Card;

pub fn part1(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input
        .lines()
        .map(str::parse)
        .map(Result::ok)
        .collect::<Option<_>>()?;

    let points = cards.iter().map(|card| {
        let my_winning_nums = card
            .nums
            .iter()
            .filter(|num| card.winning_nums.contains(num))
            .count();
        if my_winning_nums != 0 {
            2usize.pow(my_winning_nums as u32 - 1)
        } else {
            0
        }
    });

    Some(points.sum())
}

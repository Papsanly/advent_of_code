use crate::card::Card;

pub fn part2(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input
        .lines()
        .map(str::parse)
        .map(Result::ok)
        .collect::<Option<_>>()?;

    let mut card_counts = vec![1; cards.len()];

    for card in cards {
        let winnings = card.get_winning_count();
        for i in 0..winnings {
            card_counts[card.id + i] += card_counts[card.id - 1];
        }
        println!("{:?}", card_counts);
    }

    Some(card_counts.iter().sum())
}

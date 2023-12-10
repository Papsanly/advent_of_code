pub fn part1(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let seeds: Vec<usize> = lines
        .next()?
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().ok())
        .collect::<Option<_>>()?;

    dbg!(&seeds);

    None
}

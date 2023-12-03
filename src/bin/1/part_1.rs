use crate::INPUT;

pub fn part_1() -> Option<u32> {
    INPUT
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_numeric());
            let first_digit = digits.clone().next()?.to_digit(10)?;
            let second_digit = digits.next_back()?.to_digit(10)?;
            Some(10 * first_digit + second_digit)
        })
        .sum::<Option<u32>>()
    // .collect::<Option<Vec<u32>>>()
}


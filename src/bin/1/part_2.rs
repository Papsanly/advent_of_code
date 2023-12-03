use crate::INPUT;

const DIGIT_LITERALS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_digit_literal(string: &str, r: bool) -> Option<u32> {
    let digit_literals_pos = DIGIT_LITERALS.iter().enumerate().filter_map(|(i, digit)| {
        if r {
            string.rfind(digit)
        } else {
            string.find(digit)
        }
        .map(|pos| (i + 1, pos))
    });

    let minmax_digit_literal = if r {
        digit_literals_pos.max_by_key(|(_, digit)| *digit)
    } else {
        digit_literals_pos.min_by_key(|(_, digit)| *digit)
    }?;

    Some(minmax_digit_literal.0 as u32)
}

fn parse(line: &str, r: bool) -> Option<u32> {
    let digit_pos = if r {
        line.rfind(|c: char| c.is_ascii_digit()).unwrap_or(0)
    } else {
        line.find(|c: char| c.is_ascii_digit())
            .unwrap_or(line.len())
    };
    let digit_literal = find_first_digit_literal(
        &line[if r {
            digit_pos..line.len()
        } else {
            0..digit_pos
        }],
        r,
    );
    digit_literal.or_else(|| line[digit_pos..digit_pos + 1].parse::<u32>().ok())
}

pub fn part_2() -> Option<u32> {
    INPUT
        .lines()
        .map(|line| {
            let first_digit = parse(line, false)?;
            let last_digit = parse(line, true)?;
            Some(10 * first_digit + last_digit)
        })
        .sum::<Option<u32>>()
}

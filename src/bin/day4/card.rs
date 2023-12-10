use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub winning_nums: Vec<usize>,
    pub nums: Vec<usize>,
}

impl Card {
    pub fn get_winning_count(&self) -> usize {
        self.nums
            .iter()
            .filter(|num| self.winning_nums.contains(num))
            .count()
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+): ([^|]+) \| ([^|]+)").map_err(|_| ())?;
        let (_, [id, winning_nums, nums]) =
            re.captures_iter(s).map(|c| c.extract()).next().ok_or(())?;

        let [winning_nums, nums] = [winning_nums, nums].map(|nums| {
            nums.split_whitespace()
                .map(|num| num.parse().map_err(|_| ()))
                .collect::<Result<_, _>>()
        });

        Ok(Card {
            id: id.parse().map_err(|_| ())?,
            winning_nums: winning_nums?,
            nums: nums?,
        })
    }
}

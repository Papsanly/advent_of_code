mod gear;
mod part1;
mod part2;
mod part_number;
mod schematic;

use part1::part1;
use part2::part2;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part day1: {:?}", part1(INPUT));
    println!("Part day2: {:?}", part2(INPUT));
}

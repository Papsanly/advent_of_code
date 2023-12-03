mod part_1;
mod part_2;
use part_1::part_1;
use part_2::part_2;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let res_1 = part_1();
    let res_2 = part_2();
    println!("Part 1: {res_1:?}");
    println!("Part 2: {res_2:?}");
}

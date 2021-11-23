use aoc_2020::day03::*;

pub const INPUT: &str = include_str!("../inputs/03");

pub const SLOPE_RUN: usize = 3;
pub const SLOPE_RISE: usize = 1;

fn main() {
    let field = Field::new(INPUT);

    println!("Part 1:");
    let trees = field.traverse_cycle(1, 3);
    // let trees = field.traverse_collect(1, 3);
    println!("\tFound\t{} trees", trees);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(Field::new(INPUT).traverse_cycle(1, 3), 148);
        assert_eq!(Field::new(INPUT).traverse_collect(1, 3), 148)
    }
}

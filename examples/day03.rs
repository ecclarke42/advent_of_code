use aoc_2020::day03::*;

pub const INPUT: &str = include_str!("../inputs/03");

pub const SLOPE_RUN: usize = 3;
pub const SLOPE_RISE: usize = 1;

fn main() {
    let field = Field::new(INPUT);

    println!("Part 1:");
    let path_1_3 = field.traverse_cycle(1, 3);
    // let trees = field.traverse_collect(1, 3);
    println!("\tFound\t{} trees", path_1_3);

    println!("Part 2:");
    let path_1_1 = field.traverse_cycle(1, 1);
    // let path_1_3 = field.traverse_cycle(1, 1);
    let path_1_5 = field.traverse_cycle(1, 5);
    let path_1_7 = field.traverse_cycle(1, 7);
    let path_2_1 = field.traverse_cycle(2, 1);
    // let run_1_1 = field.traverse_cycle(1, 1);
    println!("\tRight 1, Down 1: \t{} trees", path_1_1);
    println!("\tRight 3, Down 1: \t{} trees", path_1_3);
    println!("\tRight 5, Down 1: \t{} trees", path_1_5);
    println!("\tRight 7, Down 1: \t{} trees", path_1_7);
    println!("\tRight 1, Down 2: \t{} trees", path_2_1);
    println!();
    println!(
        "\tProduct = {}",
        path_1_1 * path_1_3 * path_1_5 * path_1_7 * path_2_1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let field = Field::new(INPUT);
        assert_eq!(field.traverse_cycle(1, 3), 148);
        assert_eq!(field.traverse_collect(1, 3), 148)
    }

    #[test]
    fn part_2() {
        let field = Field::new(INPUT);
        assert_eq!(field.traverse_cycle(1, 1), 50);
        assert_eq!(field.traverse_cycle(1, 3), 148);
        assert_eq!(field.traverse_cycle(1, 5), 53);
        assert_eq!(field.traverse_cycle(1, 7), 64);
        assert_eq!(field.traverse_cycle(2, 1), 29);
    }
}

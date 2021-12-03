use aoc_2020::day07::*;

pub const INPUT: &str = include_str!("../inputs/07");

fn main() {
    let rules = INPUT
        .lines()
        .map(|input| Rule::try_from(input).expect("Failed to parse"))
        .collect::<Tree<'_>>();

    let my_bag = Color("shiny gold");

    println!("Part 1:");
    let sum = rules
        .bags_that_can_contain(&my_bag)
        .expect("Shiny gold not found!")
        .count();

    println!("\tTotal: {}", sum);

    println!("Part 2:");
    let sum = rules.count_contents_of(&my_bag).expect("Lost luggage");
    println!("\tSum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            INPUT
                .lines()
                .map(|input| Rule::try_from(input).expect("Failed to parse"))
                .collect::<Tree<'_>>()
                .bags_that_can_contain(&Color("shiny gold"))
                .expect("Shiny gold not found!")
                .count(),
            348
        );
    }

    // #[test]
    // fn part_2() {
    //     assert_eq!(todo!(), 3229);
    // }
}

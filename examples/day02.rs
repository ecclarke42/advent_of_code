use aoc_2020::day02::*;

pub const INPUT: &str = include_str!("../inputs/02");

fn main() {
    let entries = parse_input();

    println!("Part 1:");
    let valid = entries.clone().filter(Entry::validate_1).count();
    println!("\tFound\t{} valid entries", valid);

    println!("Part 2:");
    let valid = entries.filter(Entry::validate_2).count();
    println!("\tFound\t{} valid entries", valid);
}

fn parse_input() -> impl Iterator<Item = Entry<'static>> + Clone {
    Entry::parse_naive(INPUT).map(|entry| entry.expect("Failed to parse input"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(parse_input().filter(Entry::validate_1).count(), 660)
    }

    #[test]
    fn part_2() {
        assert_eq!(parse_input().filter(Entry::validate_2).count(), 530)
    }
}

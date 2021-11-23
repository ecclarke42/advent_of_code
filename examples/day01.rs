use aoc_2020::day01::*;

const INPUT: &str = include_str!("../inputs/01");
const TARGET_SUM: usize = 2020;

fn main() {
    let inputs = parse_input();

    println!("Part 1:");
    let (x, y) = find_pair_with_sum(&inputs, TARGET_SUM).expect("No pair found");
    println!("\tFound\t{} + {} = {}", x, y, TARGET_SUM);
    println!("\t\t{} * {} = {}", x, y, x * y);

    println!("Part 2:");
    let (x, y, z) = find_triple_with_sum(&inputs, TARGET_SUM).expect("No triple found");
    println!("\tFound\t{} + {} + {} = {}", x, y, z, TARGET_SUM);
    println!("\t\t{} * {} * {} = {}", x, y, z, x * y * z);
}

fn parse_input() -> Vec<usize> {
    INPUT
        .lines()
        .map(|line| line.parse().expect("Failed to parse"))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pair_naive_works() {
        let inputs = parse_input();
        let (x, y) = find_pair_with_sum_naive(&inputs, TARGET_SUM).expect("Not Found");
        assert_eq!(x * y, 913824);
    }

    #[test]
    fn pair_iter_works() {
        let inputs = parse_input();
        let (x, y) = find_pair_with_sum(&inputs, TARGET_SUM).expect("Not Found");
        assert_eq!(x * y, 913824);
    }

    #[test]
    fn triple_works() {
        let inputs = parse_input();
        let (x, y, z) = find_triple_with_sum(&inputs, TARGET_SUM).expect("Not Found");
        assert_eq!(x * y * z, 240889536);
    }
}

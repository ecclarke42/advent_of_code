use aoc_2020::day02::*;

fn main() {
    let entries = Entry::parse_naive(INPUT).map(|entry| entry.expect("Failed to parse input"));

    println!("Part 1:");
    let valid = entries.clone().filter(Entry::validate_1).count();
    println!("\tFound\t{} valid entries", valid);

    println!("Part 2:");
    let valid = entries.clone().filter(Entry::validate_2).count();
    println!("\tFound\t{} valid entries", valid);
}

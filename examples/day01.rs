use aoc_2020::day01::*;

fn main() {
    println!("Part 1:");
    let (x, y) = find_pair_with_sum(&INPUTS, TARGET_SUM).expect("No pair found");
    println!("\tFound\t{} + {} = {}", x, y, TARGET_SUM);
    println!("\t\t{} * {} = {}", x, y, x * y);

    println!("Part 2:");
    let (x, y, z) = find_triple_with_sum(&INPUTS, TARGET_SUM).expect("No triple found");
    println!("\tFound\t{} + {} + {} = {}", x, y, z, TARGET_SUM);
    println!("\t\t{} * {} * {} = {}", x, y, z, x * y * z);
}

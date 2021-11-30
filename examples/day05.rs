use std::{collections::BTreeSet, str::FromStr};

use aoc_2020::day05::*;

pub const INPUT: &str = include_str!("../inputs/05");

fn main() {
    let seat_ids = INPUT
        .lines()
        .filter_map(|line| BoardingPass::from_str(line).ok())
        .map(|bp| bp.seat_id());

    println!("Part 1:");
    let max = seat_ids.clone().max().expect("No max found");

    println!("\tFound max seat id: {}", max);

    println!("Part 2:");
    let ids = seat_ids.collect::<BTreeSet<_>>();
    let (before, _after) = ids
        .iter()
        .zip(ids.iter().skip(1))
        .find(|(&a, &b)| (b - a) > 1)
        .expect("Failed to find seat");

    println!("\tFound missing seat id: {}", before + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            INPUT
                .lines()
                .filter_map(|line| BoardingPass::from_str(line).ok())
                .map(|bp| bp.seat_id())
                .max()
                .expect("No max found"),
            933
        );
    }

    #[test]
    fn part_2() {
        let ids = INPUT
            .lines()
            .filter_map(|line| BoardingPass::from_str(line).ok())
            .map(|bp| bp.seat_id())
            .collect::<BTreeSet<_>>();

        let (before, _after) = ids
            .iter()
            .zip(ids.iter().skip(1))
            .find(|(&a, &b)| (b - a) > 1)
            .expect("Failed to find seat");

        assert_eq!(before + 1, 711);
    }
}

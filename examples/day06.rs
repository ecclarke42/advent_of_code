use std::collections::HashSet;

// use aoc_2020::day06::*;

pub const INPUT: &str = include_str!("../inputs/06");

fn main() {
    let groups = INPUT.split("\n\n");

    println!("Part 1:");
    let sum = groups
        .clone()
        .map(|group| {
            group
                .lines()
                .fold(HashSet::new(), |mut answers, person_response| {
                    answers.extend(person_response.chars());
                    answers
                })
                .len()
        })
        .sum::<usize>();

    println!("\tSum: {}", sum);

    println!("Part 2:");
    let sum = groups
        .map(|group| {
            let mut responses = group
                .lines()
                .map(|person_response| person_response.chars().collect::<HashSet<_>>());
            let mut all = responses.next().expect("No groups");
            for next in responses {
                all = all.intersection(&next).cloned().collect();
            }
            all.len()
        })
        .sum::<usize>();
    println!("\tSum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part_1() {
    //     assert_eq!(todo!(), 6437);
    // }

    // #[test]
    // fn part_2() {
    //     assert_eq!(todo!(), 3229);
    // }
}

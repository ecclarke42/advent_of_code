use aoc_2020::day04::*;
use validator::Validate;

pub const INPUT: &str = include_str!("../inputs/04");

fn main() {
    let raw = RawPassport::parse_iter_from(INPUT).filter_map(Result::ok);

    println!("Part 1:");
    let valid = raw.clone().count();
    println!("\tFound {} valid passports", valid);

    println!("Part 2:");
    let valid = raw
        .filter_map(|r| {
            Passport::try_from(r)
                .ok()
                .map(|p| p.validate().map(|_| p).ok())
                .flatten()
        })
        .count();

    println!("\tFound {} valid passports", valid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            RawPassport::parse_iter_from(INPUT)
                .filter_map(Result::ok)
                .count(),
            260
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            Passport::parse_iter_from(INPUT)
                .filter_map(|p| p.ok().map(|p| p.validate().ok()).flatten())
                .count(),
            153
        );
    }
}

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
        .filter_map(|r| Passport::try_from(r).ok())
        .filter(|p| p.validate().is_ok())
        .count();

    // TODO: Getting 155, but this is wrong

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
}

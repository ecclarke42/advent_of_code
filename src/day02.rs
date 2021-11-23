const INPUT: &str = include_str!("inputs/02");

struct Entry<'a> {
    letter: char,
    min: usize,
    max: usize,

    data: &'a str,
}

impl<'a> Entry<'a> {
    pub fn validate(&self) -> bool {
        let instances = self.data.chars().filter(|d| d == &self.letter).count();
        (instances <= self.max) && (instances >= self.min)
    }
}

fn parse_line_naive(line: &str) -> Option<Entry<'_>> {
    let mut parts = line.split_whitespace();

    let (min, max) = parts.next()?.split_once('-')?;
    let letter = parts.next()?.trim_end_matches(':').chars().next()?;
    let data = parts.next()?;

    Some(Entry {
        letter,
        min: min.parse().ok()?,
        max: max.parse().ok()?,
        data,
    })
}

pub fn naive() -> usize {
    INPUT
        .split('\n')
        .filter(|line| {
            parse_line_naive(line)
                .expect("Failed to parse line")
                .validate()
        })
        .count()
}

pub const INPUT: &str = include_str!("inputs/02");

/// An entry in the form: a-b x: abcdef
pub struct Entry<'a> {
    letter: char,
    a: usize,
    b: usize,

    data: &'a str,
}

// TODO: FromStr isn't happy about lifetimes
// impl<'a> FromStr for Entry<'a> {
// type Err = ParseError;
// fn from_str(s: &str) -> Result<Self, Self::Err> {
impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = ParseError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut parts = s.split_whitespace();

        let (a, b) = parts
            .next()
            .ok_or(ParseError::NotEnough)?
            .split_once('-')
            .ok_or(ParseError::SplitAB)?;

        let letter = parts
            .next()
            .ok_or(ParseError::NotEnough)?
            .trim_end_matches(':')
            .chars()
            .next()
            .ok_or(ParseError::NoLetter)?;

        let data = parts.next().ok_or(ParseError::NotEnough)?;

        Ok(Entry {
            letter,
            a: a.parse()?,
            b: b.parse()?,
            data,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Not enough data in the input")]
    NotEnough,

    #[error("Failed to split \"a-b\"")]
    SplitAB,

    #[error("Missing letter argument")]
    NoLetter,

    #[error("Failed to parse a/b as integers: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

impl<'a> Entry<'a> {
    pub fn parse_naive(
        input: &'a str,
    ) -> impl Iterator<Item = Result<Entry<'a>, ParseError>> + Clone {
        input.split('\n').map(Self::try_from)
    }

    /// Validate for part 1, where `a` is character min, `b` is character max
    pub fn validate_1(&self) -> bool {
        let min = self.a;
        let max = self.b;
        let instances = self.data.chars().filter(|d| d == &self.letter).count();
        (instances <= max) && (instances >= min)
    }

    /// Validate for part 2, where `a` and `b` are positions in `data` where the
    /// entry is valid if exactly one of those contains `letter`.
    ///
    /// NOTE: `a` and `b` are 1-indexed
    pub fn validate_2(&self) -> bool {
        let a = self
            .data
            .chars()
            .nth(self.a - 1)
            .map(|c| c == self.letter)
            .expect("a");

        let b = self
            .data
            .chars()
            .nth(self.b - 1)
            .map(|c| c == self.letter)
            .expect("b");

        a ^ b
    }
}

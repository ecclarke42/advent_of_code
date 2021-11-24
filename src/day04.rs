use std::str::FromStr;

use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Deserialize)]
pub struct RawPassport<'a> {
    #[serde(rename = "byr")]
    birth_year: &'a str,

    #[serde(rename = "iyr")]
    issue_year: &'a str,

    #[serde(rename = "eyr")]
    expiration_year: &'a str,

    #[serde(rename = "hgt")]
    height: &'a str,

    #[serde(rename = "hcl")]
    hair_color: &'a str,

    #[serde(rename = "ecl")]
    eye_color: &'a str,

    #[serde(rename = "pid")]
    passport_id: &'a str,

    #[serde(rename = "cid")]
    country_id: Option<&'a str>,
}

#[derive(Debug, /*Deserialize,*/ Validate)]
pub struct Passport<'a> {
    // #[serde(rename = "byr")]
    #[validate(range(min = 1920, max = 2002))]
    birth_year: i32,

    // #[serde(rename = "iyr")]
    #[validate(range(min = 2010, max = 2020))]
    issue_year: i32,

    // #[serde(rename = "eyr")]
    #[validate(range(min = 2020, max = 2030))]
    expiration_year: i32,

    // #[serde(rename = "hgt")]
    height: Height,

    // #[serde(rename = "hcl")]
    hair_color: HexColor,

    // #[serde(rename = "ecl")]
    eye_color: EyeColor,

    // #[serde(rename = "pid")]
    passport_id: Id<'a>,

    // #[serde(rename = "cid")]
    country_id: Option<&'a str>,
}

#[derive(Debug)]
pub struct Height {
    value: u32,
    units: LengthUnit,
}

#[derive(Debug)]
pub enum LengthUnit {
    Cm,
    In,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseHeightError {
    #[error("failed to parse: {0}")]
    Value(#[from] std::num::ParseIntError),

    #[error("unknown unit, or no units")]
    UnknownUnit,
}

impl Validate for Height {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = None;
        match self.units {
            LengthUnit::Cm => {
                if self.value < 150 {
                    errors
                        .get_or_insert(ValidationErrors::new())
                        .add("value", ValidationError::new("cm length too short"));
                } else if self.value > 193 {
                    errors
                        .get_or_insert(ValidationErrors::new())
                        .add("value", ValidationError::new("cm length too long"));
                }
            }
            LengthUnit::In => {
                if self.value < 59 {
                    errors
                        .get_or_insert(ValidationErrors::new())
                        .add("value", ValidationError::new("inch length too short"));
                } else if self.value > 76 {
                    errors
                        .get_or_insert(ValidationErrors::new())
                        .add("value", ValidationError::new("inch length too long"));
                }
            }
        }

        if let Some(errors) = errors {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

impl FromStr for Height {
    type Err = ParseHeightError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, units) = if let Some(value) = s.strip_suffix("cm") {
            (value, LengthUnit::Cm)
        } else if let Some(value) = s.strip_suffix("in") {
            (value, LengthUnit::In)
        } else {
            return Err(ParseHeightError::UnknownUnit);
        };

        Ok(Self {
            value: value.parse()?,
            units,
        })
    }
}

#[derive(Debug)]
pub struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for HexColor {
    type Err = ParseHexColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 7 {
            return Err(ParseHexColorError::NotEnough);
        }

        if s.chars()
            .next()
            .ok_or(ParseHexColorError::NotEnough)?
            .ne(&'#')
        {
            return Err(ParseHexColorError::NoHash);
        }

        let mut bytes = [0u8; 3];
        hex::decode_to_slice(&s[1..], &mut bytes)?;

        Ok(Self {
            red: bytes[0],
            blue: bytes[1],
            green: bytes[2],
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseHexColorError {
    #[error("Not enough data in the input")]
    NotEnough,
    #[error("Missing # symbol")]
    NoHash,
    #[error("Failed to decode input")]
    Decode(#[from] hex::FromHexError),
}

#[derive(Debug)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "amb" => EyeColor::Amber,
            "blu" => EyeColor::Blue,
            "brn" => EyeColor::Brown,
            "gry" => EyeColor::Gray,
            "grn" => EyeColor::Green,
            "hzl" => EyeColor::Hazel,
            "oth" => EyeColor::Other,
            unknown => return Err(unknown.to_string()),
        })
    }
}

#[derive(Debug)]
pub struct Id<'a>(&'a str);

impl<'a> TryFrom<&'a str> for Id<'a> {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.chars().count() != 9 {
            Err(())
        } else {
            Ok(Id(value))
        }
    }
}

impl<'a> TryFrom<&'a str> for RawPassport<'a> {
    type Error = PassportKeyError<'a>;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut birth_year = None;
        let mut issue_year = None;
        let mut expiration_year = None;
        let mut height = None;
        let mut hair_color = None;
        let mut eye_color = None;
        let mut passport_id = None;
        let mut country_id = None;

        for pair in s.split_whitespace().map(|s| s.split_once(':')) {
            let (key, value) = pair.ok_or(PassportKeyError::MalformedKeyValue)?;
            match key {
                "byr" => birth_year = Some(value),
                "iyr" => issue_year = Some(value),
                "eyr" => expiration_year = Some(value),
                "hgt" => height = Some(value),
                "hcl" => hair_color = Some(value),
                "ecl" => eye_color = Some(value),
                "pid" => passport_id = Some(value),
                "cid" => country_id = Some(value),
                other => return Err(PassportKeyError::UnknownKey(other)),
            }
        }

        Ok(Self {
            birth_year: birth_year.ok_or(PassportKeyError::MissingBirthYear)?,
            issue_year: issue_year.ok_or(PassportKeyError::MissingIssueYear)?,
            expiration_year: expiration_year.ok_or(PassportKeyError::MissingExpirationYear)?,
            height: height.ok_or(PassportKeyError::MissingHeight)?,
            hair_color: hair_color.ok_or(PassportKeyError::MissingHairColor)?,
            eye_color: eye_color.ok_or(PassportKeyError::MissingEyeColor)?,
            passport_id: passport_id.ok_or(PassportKeyError::MissingPassportId)?,
            country_id,
        })
    }
}

pub enum PassportKeyError<'a> {
    MissingBirthYear,
    MissingIssueYear,
    MissingExpirationYear,
    MissingHeight,
    MissingHairColor,
    MissingEyeColor,
    MissingPassportId,

    MalformedKeyValue,
    UnknownKey(&'a str),
}

impl<'a> RawPassport<'a> {
    pub fn parse_iter_from(
        s: &'a str,
    ) -> impl Iterator<Item = Result<RawPassport<'a>, PassportKeyError<'a>>> + Clone {
        s.split("\n\n").map(RawPassport::try_from)
    }
}

impl<'a> Passport<'a> {
    pub fn parse_iter_from(
        s: &'a str,
    ) -> impl Iterator<Item = Result<Passport<'a>, PassportParseError<'a>>> + Clone {
        RawPassport::parse_iter_from(s).map(|r| match r {
            Ok(raw) => Passport::try_from(raw).map_err(PassportParseError::Value),
            Err(e) => Err(PassportParseError::Key(e)),
        })
    }
}

pub enum PassportParseError<'a> {
    Key(PassportKeyError<'a>),
    Value(PassportValueError),
}

impl<'a> TryFrom<RawPassport<'a>> for Passport<'a> {
    type Error = PassportValueError;
    fn try_from(
        RawPassport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        }: RawPassport<'a>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            birth_year: birth_year.parse().map_err(PassportValueError::BirthYear)?,
            issue_year: issue_year.parse().map_err(PassportValueError::IssueYear)?,
            expiration_year: expiration_year
                .parse()
                .map_err(PassportValueError::ExpirationYear)?,

            height: height.parse().map_err(PassportValueError::Height)?,
            hair_color: hair_color.parse().map_err(PassportValueError::HairColor)?,
            eye_color: eye_color.parse().map_err(PassportValueError::EyeColor)?,
            passport_id: Id::try_from(passport_id).map_err(|_| PassportValueError::Id)?,
            country_id,
        })
    }
}

pub enum PassportValueError {
    BirthYear(std::num::ParseIntError),
    IssueYear(std::num::ParseIntError),
    ExpirationYear(std::num::ParseIntError),
    Height(ParseHeightError),
    HairColor(ParseHexColorError),
    EyeColor(String),
    Id,
}

// pub mod serde_passport {

//     #[derive(Debug, thiserror::Error)]
//     pub enum Error {
//         #[error("After finishing deserialization, there are characters left in the input")]
//         TrailingCharacters,

//         #[error("Custom error: {0}")]
//         Custom(String),
//     }

//     impl serde::de::Error for Error {
//         fn custom<T>(msg: T) -> Self
//         where
//             T: std::fmt::Display,
//         {
//             Self::Custom(msg.to_string())
//         }
//     }

//     pub struct Deserializer<'de> {
//         input: &'de str,
//     }

//     impl<'de> Deserializer<'de> {
//         pub fn from_str(input: &'de str) -> Self {
//             Deserializer { input }
//         }
//     }

//     pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
//     where
//         T: serde::Deserialize<'a>,
//     {
//         let mut deserializer = Deserializer::from_str(s);
//         let t = T::deserialize(&mut deserializer)?;
//         if deserializer.input.is_empty() {
//             Ok(t)
//         } else {
//             Err(Error::TrailingCharacters)
//         }
//     }

//     impl<'de> Deserializer<'de> {
//         fn parse_bool(&self,
//     }

//     impl<'de, 'a> serde::Deserializer<'de> for &'a mut Deserializer<'de> {
//         type Error = Error;

//         fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//         where
//             V: serde::de::Visitor<'de>,
//         {

//             visitor.visit_str(v)
//         }

//         fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//         where
//                 V: serde::de::Visitor<'de> {

//         }
//     }
// }

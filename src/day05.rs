use std::str::FromStr;

pub struct BoardingPass(u8, u8);

impl BoardingPass {
    pub fn row(&self) -> u8 {
        self.0
    }
    pub fn col(&self) -> u8 {
        self.1
    }

    pub fn seat_id(&self) -> u16 {
        let row: u16 = self.0.into();
        let col: u16 = self.1.into();

        row * 8 + col
    }
}

impl FromStr for BoardingPass {
    type Err = BoardingPassParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ops = s.chars();

        // Partition Rows
        let mut r0 = 0u8;
        let mut r1 = 127u8;

        // Partition the range until we have r0 + 1 = r1
        for _ in 0..6 {
            let mid = r0 + ((r1 - r0) / 2);
            match ops.next().ok_or(BoardingPassParseError::NotEnough)? {
                'F' => r1 = mid,
                'B' => r0 = mid + 1,
                other => return Err(BoardingPassParseError::UnknownOp(other)),
            }
        }

        // Use the 7th partition to determine which to use
        let row = match ops.next().ok_or(BoardingPassParseError::NotEnough)? {
            'F' => r0,
            'B' => r1,
            other => return Err(BoardingPassParseError::UnknownOp(other)),
        };

        // Partion Columns
        let mut c0 = 0u8;
        let mut c1 = 7u8;

        for _ in 0..2 {
            let mid = c0 + ((c1 - c0) / 2);
            match ops.next().ok_or(BoardingPassParseError::NotEnough)? {
                'L' => c1 = mid,
                'R' => c0 = mid + 1,
                other => return Err(BoardingPassParseError::UnknownOp(other)),
            }
        }

        // Use the 3rd partition to determine which to use
        let col = match ops.next().ok_or(BoardingPassParseError::NotEnough)? {
            'L' => c0,
            'R' => c1,
            other => return Err(BoardingPassParseError::UnknownOp(other)),
        };

        if ops.next().is_some() {
            return Err(BoardingPassParseError::TooMuch);
        }

        Ok(Self(row, col))
    }
}

#[derive(Debug)]
pub enum BoardingPassParseError {
    NotEnough,
    TooMuch,

    UnknownOp(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode() {
        let pass = BoardingPass::from_str("FBFBBFFRLR").expect("Failed to parse");
        assert_eq!(pass.row(), 44);
        assert_eq!(pass.col(), 5);
        assert_eq!(pass.seat_id(), 357);

        let pass = BoardingPass::from_str("BFFFBBFRRR").expect("Failed to parse");
        assert_eq!(pass.row(), 70);
        assert_eq!(pass.col(), 7);
        assert_eq!(pass.seat_id(), 567);

        let pass = BoardingPass::from_str("FFFBBBFRRR").expect("Failed to parse");
        assert_eq!(pass.row(), 14);
        assert_eq!(pass.col(), 7);
        assert_eq!(pass.seat_id(), 119);

        let pass = BoardingPass::from_str("BBFFBBFRLL").expect("Failed to parse");
        assert_eq!(pass.row(), 102);
        assert_eq!(pass.col(), 4);
        assert_eq!(pass.seat_id(), 820);
    }
}

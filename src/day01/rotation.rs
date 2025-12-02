use std::str::FromStr;

pub enum Rotation {
    Left(u32),
    Right(u32),
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clicks = s[1..].parse::<u32>().unwrap();
        if s.chars().nth(0) == Some('L') {
            Ok(Rotation::Left(clicks))
        } else if s.chars().nth(0) == Some('R') {
            Ok(Rotation::Right(clicks))
        } else {
            Err(())
        }
    }
}


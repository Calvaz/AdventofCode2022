use Rps::*;
use Points::*;
use std::str::FromStr;

#[derive(PartialEq)]
pub enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub enum Points {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Rps {

    type Err = ();

    fn from_str(input: &str) -> Result<Rps, Self::Err> {
        match input {
            "A"  => Ok(Rps::Rock),
            "X"  => Ok(Rps::Rock),
            "B"  => Ok(Rps::Paper),
            "Y"  => Ok(Rps::Paper),
            "C"  => Ok(Rps::Scissors),
            "Z"  => Ok(Rps::Scissors),
            _    => Err(()),
        }
    }
}

impl Rps {

    pub fn challenge((player, opponent): (Rps, Rps)) -> i32 {
        match (player, opponent) {
            (Rock, Scissors)    => Rock as i32 + Win as i32,
            (Rock, Paper)       => Rock as i32 + Lose as i32,
            (Rock, Rock)        => Rock as i32 + Draw as i32,
            (Paper, Scissors)   => Paper as i32 + Lose as i32,
            (Paper, Paper)      => Paper as i32 + Draw as i32,
            (Paper, Rock)       => Paper as i32 + Win as i32,
            (Scissors, Scissors) => Scissors as i32 + Draw as i32,
            (Scissors, Paper)   => Scissors as i32 + Win as i32,
            (Scissors, Rock)    => Scissors as i32 + Lose as i32,
        }
    }
}
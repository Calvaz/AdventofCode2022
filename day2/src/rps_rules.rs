use Rps::*;
use Points::*;
use std::str::FromStr;

pub enum Points {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Points {

    type Err = ();

    fn from_str(input: &str) -> Result<Points, Self::Err> {
        match input {
            "X"  => Ok(Lose),
            "Y"  => Ok(Draw),
            "Z"  => Ok(Win),
            _    => Err(()),
        }
    }
}

#[derive(PartialEq)]
pub enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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

    pub fn challenge_part_one((player, opponent): (Rps, Rps)) -> i32 {
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

    pub fn challenge_part_two((tactic, opponent): (Points, Rps)) -> i32 {
        match (tactic, opponent) {
            (Draw, Scissors)    => Scissors as i32 + Draw as i32,
            (Lose, Scissors)    => Paper as i32 + Lose as i32,
            (Win, Scissors)     => Rock as i32 + Win as i32,
            (Draw, Rock)        => Rock as i32 + Draw as i32,
            (Lose, Rock)        => Scissors as i32 + Lose as i32,
            (Win, Rock)         => Paper as i32 + Win as i32,
            (Draw, Paper)       => Paper as i32 + Draw as i32,
            (Lose, Paper)       => Rock as i32 + Lose as i32,
            (Win, Paper)        => Scissors as i32 + Win as i32,
        }
    }
}
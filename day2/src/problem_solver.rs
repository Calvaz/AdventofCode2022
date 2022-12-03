use crate::rps_rules::Rps;
use std::fs;
use std::str::FromStr;

pub struct ProblemSolver {}

impl ProblemSolver {

    pub fn solve() {
        let content = fs::read_to_string("./input").unwrap_or_default();
        Self::solve_with_content(&content);
    }

    fn solve_with_content(content: &str) -> i32 {
        let games = content.lines();
        let mut score: i32 = 0;
        
        for g in games {
            let players: Vec<&str> = g.split(' ').collect();
            let player = Rps::from_str(&players[1]).unwrap();
            let opponent = Rps::from_str(&players[0]).unwrap();

            score += Rps::challenge((player, opponent));
        }

        println!("{}", score);
        score
    }
}

// test does not work cause of unwrap()
#[cfg(test)]
mod tests {

    use crate::ProblemSolver;
    #[test]
    fn solve_test() {
        let content = r"A Y
        B X
        C Z";

        let score = ProblemSolver::solve_with_content(content);
        assert_eq!(15, score);
    }
}

use std::fs;

pub struct PuzzleSolver {}

impl PuzzleSolver { 

    pub fn solve(&self) -> i32 {
        let content = fs::read_to_string("./input").unwrap_or_default();
        Self::solve_puzzle(&content)
    }

    pub fn solve_puzzle(content: &str) -> i32 {
        let puzzle = content.lines();

        let mut priority_sum: i32 = 0;
        for r in puzzle {
            let (first_half, second_half) = Self::split_half(r);
            
        }

        1
    }

    fn split_half(row: &str) -> (Vec<char>, Vec<char>) {
        let mut chars = row.chars();
        let first_half: Vec<char> = chars.by_ref().take(row.len() / 2).collect();
        let second_half: Vec<char> = chars.by_ref().collect();
        (first_half, second_half)
    }

    fn is_char_present(first_half: Vec<char>, second_half: Vec<char>) {
        for c in first_half {
            if second_half.contains(&c) {

            }
        }
    }

}

#[cfg(test)]
mod tests {

    use crate::PuzzleSolver;

    #[test]
    fn split_half_test() {
        let sentence = "HelloworlD";
        let result = PuzzleSolver::split_half(sentence);

        let hello: Vec<char> = vec!['H', 'e', 'l', 'l', 'o'];
        let world: Vec<char> = vec!['w', 'o', 'r', 'l', 'D'];
        assert_eq!((hello, world), result);
    }

    #[test]
    fn test_char_value() {
        assert_eq!('A' as u8, 1);
    }
}


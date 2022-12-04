use std::fs;

pub struct PuzzleSolver {}

impl PuzzleSolver { 

    pub fn solve() -> i32 {
        let content = fs::read_to_string("./input").unwrap_or_default();
        Self::solve_puzzle(&content)
    }

    pub fn solve_puzzle(content: &str) -> i32 {
        let puzzle = content.lines();

        let mut priority_sum: i32 = 0;
        for r in puzzle {
            let (first_half, second_half) = Self::split_half(r);
            priority_sum += Self::calculate_char_priority(first_half, second_half);
        }
        priority_sum
    }

    fn split_half(row: &str) -> (Vec<char>, Vec<char>) {
        let mut chars = row.chars();
        let first_half: Vec<char> = chars.by_ref().take(row.len() / 2).collect();
        let second_half: Vec<char> = chars.by_ref().collect();
        (first_half, second_half)
    }

    // could use a hashmap but I didn't want to
    fn calculate_char_priority(first_half: Vec<char>, second_half: Vec<char>) -> i32 {
        let mut ret: i32 = 0;
        for c in first_half {
            if second_half.contains(&c) {
                if c.is_ascii_uppercase() {
                    ret = ((c as i32 + 13) % 52) + 1;  // + 1 because modulus gives 0 rest if correct division
                    break;
                }
                ret = ((c as i32 + 7) % 26) + 1;
                break;
            }
        }

        println!("{}", ret);
        ret
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
    fn calculate_priority_test() {
        let mut first_half: Vec<char> = vec!['v','J','r','w','p','W','t','w','J','g','W','r'];
        let mut second_half: Vec<char> = vec!['h','c','s','F','M','M','f','F','F','h','F','p'];

        let mut result: i32 = PuzzleSolver::calculate_char_priority(first_half, second_half);
        assert_eq!(result, 16);

        first_half = vec!['j','q','H','R','N','q','R','j','q','z','j','G','D','L','G','L'];
        second_half = vec!['r','s','F','M','f','F','Z','S','r','L','r','F','Z','s','S','L'];
        
        result = PuzzleSolver::calculate_char_priority(first_half, second_half);
        assert_eq!(result, 38);

        first_half = vec!['a','b','c','D'];
        second_half = vec!['f','g','h', 'a'];
        
        result = PuzzleSolver::calculate_char_priority(first_half, second_half);
        assert_eq!(result, 1);

    }
}


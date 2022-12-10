use std::fs;

fn main() {
    let file = fs::read_to_string("./input").expect("file is present");
    let result = solution::solve(file);

}

mod solution {

    // part one
    // pub fn solve(file: String) -> isize {

    // part two
    pub fn solve(file: String) -> String {        
        let content = file.lines();
    
        let mut cycles: u8 = 0;
        // part one
        // let mut x: isize = 0;

        // part two
        let mut x: isize = 1;
        let mut signal_strength: isize = 0;
        let mut sprite: String = String::new();

        for l in content {
            wait_cycles(l, &mut cycles, &mut x, &mut signal_strength, &mut sprite);
        }

        let mut result = String::new();
        for chunk in sprite.chars().collect::<Vec<char>>().chunks(40) {
            let line = chunk.into_iter().take(40).collect::<String>();
            result.push_str(&format!("{}\n", line));
        }
        
        print!("{}", result);
        result = result.split_at(result.len() - 1).0.to_string();
        result

        // part one
        // println!("{}", signal_strength);
        // signal_strength
    }

    // part one
    // fn wait_cycles(s: &str, cycles: &mut u8, x: &mut isize, signal_strength: &mut isize) -> u8 {
    
    // part two
    fn wait_cycles(s: &str, cycles: &mut u8, x: &mut isize, signal_strength: &mut isize, sprite: &mut String) -> u8 {
            let cycle_checks: Vec<u8> = vec![20, 60, 100, 140, 180, 220];
        
        for _ in 0..2 {

            *cycles += 1;
            
            if cycle_checks.contains(cycles) {
                let strength = *cycles as isize * *x;
                *signal_strength += strength;
            }

            if (sprite.len() % 40) as isize >= *x - 1 && (sprite.len() % 40) as isize <= *x + 1 {
                sprite.push_str("#");
            } else {
                sprite.push_str(".");
            }

            if s.starts_with("noop") {
                break;
            }
        }

        if s.starts_with("addx") {
            let add_op: Vec<&str> = s.split(' ').collect();
            let amount: isize = add_op[1].parse().unwrap();
            *x = *x + amount;
        }

        *cycles
    }
}

#[cfg(test)]
mod tests {
    
    use crate::solution;
    use std::fs;

    #[test]
    fn solve_test() {

        let file = fs::read_to_string("./test_input").expect("there should be the test file");
        let result = solution::solve(file);

        let mut solution = String::new();
        solution.push_str("##..##..##..##..##..##..##..##..##..##..\n");
        solution.push_str("###...###...###...###...###...###...###.\n");
        solution.push_str("####....####....####....####....####....\n");
        solution.push_str("#####.....#####.....#####.....#####.....\n");
        solution.push_str("######......######......######......####\n");
        solution.push_str("#######.......#######.......#######.....");
        
        assert_eq!(result, solution);
    }
}
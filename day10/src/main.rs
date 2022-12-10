use std::fs;

fn main() {
    let file = fs::read_to_string("./input").expect("file is present");
    let result = solution::solve(file);
    
}

mod solution {

    pub fn solve(file: String) -> isize {
        let content = file.lines();
    
        let mut cycles: u8 = 0;
        let mut x: isize = 1;
        let mut signal_strength: isize = 0;
    
        for l in content {
            wait_cycles(l, &mut cycles, &mut x, &mut signal_strength);
        }
    
        println!("{}", signal_strength);
        signal_strength
    }

    fn wait_cycles(s: &str, cycles: &mut u8, x: &mut isize, signal_strength: &mut isize) -> u8 {
        let cycle_checks: Vec<u8> = vec![20, 60, 100, 140, 180, 220];
        
        for _ in 0..2 {

            *cycles += 1;
            
            if cycle_checks.contains(cycles) {
                let strength = *cycles as isize * *x;
                *signal_strength += strength;
            }

            if s.starts_with("noop") {
                break;
            }
            
            println!("signal str: {}", signal_strength);
            println!("cycles: {}", cycles);
            println!("x: {}", x);
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

        assert_eq!(result, 13140);
    }
}
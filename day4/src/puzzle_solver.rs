use std::fs;

pub struct PuzzleSolver {}

impl PuzzleSolver { 

    pub fn solve() -> i32 {
        let content = fs::read_to_string("./input").unwrap_or_default();
        Self::solve_puzzle(&content)
    }

    pub fn solve_puzzle(content: &str) -> i32 {
        let puzzle_lines = content.lines();
        let mut assignments_contains: i32 = 0;
        
        for l in puzzle_lines {
            let elf_assignments: Vec<&str> = l.split(',').collect();
            let first_elf_task: (u16, u16) = Self::split_on_bar(elf_assignments[0]);
            let second_elf_task: (u16, u16) = Self::split_on_bar(elf_assignments[1]);
            Self::assignments_condition_second_part(&mut assignments_contains, first_elf_task, second_elf_task);
        }

        assignments_contains
    }

    fn assignments_condition_first_part(result: &mut i32, first_elf_task: (u16, u16), second_elf_task: (u16, u16)) {
        if (first_elf_task.0 >= second_elf_task.0) && (first_elf_task.1 <= second_elf_task.1) ||
            (second_elf_task.0 >= first_elf_task.0) && (second_elf_task.1 <= first_elf_task.1) {
                *result += 1;
            }
    }

    fn assignments_condition_second_part(
        result: &mut i32, 
        (first_elf_start, first_elf_end): (u16, u16), 
        (second_elf_start, second_elf_end): (u16, u16)
    ) {
        if (first_elf_start <= second_elf_start && first_elf_end >= second_elf_start ||
            first_elf_end >= second_elf_end && first_elf_start <= second_elf_end) ||
            second_elf_start <= first_elf_start && second_elf_end >= first_elf_start ||
            second_elf_end >= first_elf_end && second_elf_start <= first_elf_end { 
                *result += 1;
            }
    }

    fn split_on_bar(task: &str) -> (u16, u16) {
        let range: Vec<&str> = task.split('-').collect();
        (range[0].parse::<u16>().unwrap(), range[1].parse::<u16>().unwrap())
    }
}

#[cfg(test)]
mod tests {
    
    use crate::PuzzleSolver;

    #[test]
    fn condition_second_part() {
        let mut first_elf = (2, 4);
        let mut second_elf = (6, 8);
        let mut result: i32 = 0;

        PuzzleSolver::assignments_condition_second_part(&mut result, first_elf, second_elf);
        assert_eq!(0, result);

        first_elf = (2, 3);
        second_elf = (4, 5);
        result = 0;

        PuzzleSolver::assignments_condition_second_part(&mut result, first_elf, second_elf);
        assert_eq!(0, result);
    }
}


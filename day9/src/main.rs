use std::fs;

fn main() {
    let file = fs::read_to_string("./input").expect("file is there");
    solution::solve(file);
}

mod solution {

    pub fn solve(file: String) -> usize {
        let content = file.lines();

        let mut matrix = Vec::new();
        for _ in 0..600 {
            let mut row = Vec::new();
            for _ in 0..600 {
                row.push(0);
            }
            matrix.push(row);
        }
        
        let mut positions_visited = 0;  
        let mut head_position: (usize, usize) = (300, 300);
        let mut tail_position: (usize, usize) = (300, 300);
        for l in content {
            let command: Vec<&str> = l.split(' ').collect();
            let dir = to_direction(command[0]);
            let amount: usize = command[1].parse::<usize>().unwrap();
            
            shift(dir, amount, &mut matrix, (&mut head_position.0, &mut head_position.1), (&mut tail_position.0, &mut tail_position.1), &mut positions_visited);
        }

        println!("{}", positions_visited);
        positions_visited
    }

    pub fn shift(
        d: Direction, 
        amount: usize, 
        matrix: &mut Vec<Vec<u8>>, 
        (x, y): (&mut usize, &mut usize), 
        (tail_x, tail_y): (&mut usize, &mut usize),
        positions_visited: &mut usize
    ) {
        for _ in 0..amount {

            // tried resizing the arrays but I am not good enough
            // if *x == 0 || *y == 0 {
            //     matrix.resize(matrix.len(), Vec::new());
            //     for inner_vec in matrix.iter_mut() {
            //         inner_vec.splice(0..0, [0].iter().cloned());
            //     }

            // } else if *y > matrix[0].len() || *x > matrix[0].len() {
            //     matrix.resize(new_len: usize, value: T)
            // } 
            println!("x: {}, tail_x:{}", x, tail_x);
            println!("y: {}, tail_y:{}", y, tail_y);

            let prev = (*x, *y);
            match d {
                Direction::U => {
                    *y -= 1
                }
                Direction::R => {
                    *x += 1
                }
                Direction::D => {
                    *y += 1
                }
                Direction::L => {
                    *x -= 1
                }
            }

            // ternary operators are unstable?
            let (diff_x, diff_y) = get_tail_diff(x, y, tail_x, tail_y);

            if diff_x >= 2 || diff_y >= 2 {
                *tail_x = prev.0;
                *tail_y = prev.1;
            }
            
            // visited = true
            if matrix[*tail_y][*tail_x] != 1 {
                matrix[*tail_y][*tail_x] = 1;
                *positions_visited += 1;
            }
       }
    }

    pub fn to_direction(s: &str) -> Direction {
        match s {
            "U" => Direction::U,
            "R" => Direction::R,
            "D" => Direction::D,
            "L" => Direction::L,
            _ => panic!("Invalid direction")
        }
    }

    pub fn get_tail_diff(x: &mut usize, y: &mut usize, tail_x: &mut usize, tail_y: &mut usize) -> (isize, isize) {
        let mut diff_x: isize;
        if x > tail_x {
            diff_x = (*x - *tail_x) as isize;
        } else {
            diff_x = (*tail_x - *x) as isize;
        }
        
        let mut diff_y: isize;
        if y > tail_y {
            diff_y = (*y - *tail_y) as isize;
        } else {
            diff_y = (*tail_y - *y) as isize;
        }

        (diff_x, diff_y)
    }

    pub enum Direction {
        R,
        L,
        U,
        D
    }
}

#[cfg(test)]
mod tests {

    use crate::solution;

    #[test]
    fn solve_test() {
        let mut file = String::new();
        file.push_str("R 4\n");
        file.push_str("U 4\n");
        file.push_str("L 3\n");
        file.push_str("D 1\n");
        file.push_str("R 4\n");
        file.push_str("D 1\n");
        file.push_str("L 5\n");
        file.push_str("R 2\n");

        let result = solution::solve(file);
        assert_eq!(result, 13);
    }
}
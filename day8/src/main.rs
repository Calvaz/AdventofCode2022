use std::fs;

fn main() {
    let file = fs::read_to_string("./input").expect("there is a file");
    println!("{:?}", solution::solve(file));
}

mod solution {
    pub fn solve(file: String) -> usize {
        let content = file.lines();
    
        let rows: Vec<&str> = content.collect();
        let directions = vec![Direction::Top, Direction::Right, Direction::Bottom, Direction::Left];
        
        let rows_len = rows.len();
        let columns: Vec<char> = rows[0].chars().collect();
        let column_len: usize = columns.len(); 
        let mut trees: usize = (rows_len - 1) * 2 + (column_len - 1) * 2;

        // part two
        let mut viewable_distance: usize = 0;

        for r in 1..rows_len - 1 {
            let column: Vec<char> = rows[r].chars().collect();
            
            let column_len = rows.len();
            for c in 1..column_len - 1 {
                let tree: usize = column[c].to_digit(10).unwrap() as usize;
                let mut curr_viewable_distance: Vec<usize> = vec![0, 0, 0, 0];
                
                'dir: for d in &directions {
                    
                    // part two
                    traverse(&rows, c, r, tree, (c, r), &mut trees, &d, &mut curr_viewable_distance);


                    // part one
                    // let current_trees = trees;
                    // traverse(&rows, c, r, tree, (c, r), &mut trees, &d, );
                    // if trees > current_trees {
                    //     break 'dir;
                    // }
                }
                let curr_vd = curr_viewable_distance[0] * curr_viewable_distance[1] * curr_viewable_distance[2] * curr_viewable_distance[3];
                if curr_vd > viewable_distance {
                    viewable_distance = curr_vd;
                }
            }
        }
        
        // part one
        // println!("{:?}", trees);
        // trees

        // part two
        viewable_distance
    }
    
    fn traverse(
        height: &Vec<&str>, 
        x: usize, 
        y: usize, 
        tree: usize, 
        (tree_pos_x, tree_pos_y): (usize, usize), 
        trees: &mut usize, 
        direction: &Direction,
        viewable_distance: &mut Vec<usize>
    ) {
        let length: Vec<char> = height[y].chars().collect();
    
        let pos_value: usize = length[x].to_digit(10).unwrap() as usize;
        // println!("tree pos: {:?}, current pos: {:?}", (tree_pos_x, tree_pos_y), (x, y));
        // println!("tree value: {}, pos value: {}", tree, pos_value);
        if tree <= pos_value && (tree_pos_x != x || tree_pos_y != y) {
            return;
        }
    
        match direction {
    
            Direction::Top => {
                // println!("trying to go up with y: {:?}", y);
                if y == 0 {
                    *trees += 1;
                    // println!("trees: {:?}", trees);
                    return;
                }
    
                viewable_distance[0] += 1;
                traverse(height, x, y - 1, tree, (tree_pos_x, tree_pos_y), trees, &Direction::Top, viewable_distance);
            }
    
            Direction::Right => {
                // println!("trying to go right with x: {:?}", x);
                if x == length.len() - 1 {
                    *trees += 1;
                    // println!("trees: {:?}", trees);
                    return;
                }
    
                viewable_distance[1] += 1;
                traverse(height, x + 1, y, tree, (tree_pos_x, tree_pos_y), trees, &Direction::Right, viewable_distance);
            }
            
            Direction::Bottom => {
                // println!("trying to go down with y: {:?}", y);
                if y == height.len() - 1 {
                    *trees += 1;
                    // println!("trees: {:?}", trees);
                    return;
                }
    
                viewable_distance[2] += 1;
                traverse(height, x, y + 1, tree, (tree_pos_x, tree_pos_y), trees, &Direction::Bottom, viewable_distance);
            }
            
            Direction::Left => {
                // println!("trying to go left with x: {:?}", x);
                if x == 0 {
                    *trees += 1;
                    // println!("trees: {:?}", trees);
                    return;
                }
    
                viewable_distance[3] += 1;
                traverse(height, x - 1, y, tree, (tree_pos_x, tree_pos_y), trees, &Direction::Left, viewable_distance);
            }
        }    
    }
    
    pub enum Direction { Top, Right, Bottom, Left }
    
}

#[cfg(test)]
mod tests {
    
    use crate::solution; 
    
    // part one test
    #[test]
    fn test_solve() {
        let mut file: String = String::new();
        file.push_str("30373\n");
        file.push_str("25512\n");
        file.push_str("65332\n");
        file.push_str("33549\n");
        file.push_str("35390");
    
        let result: usize = solution::solve(file);
        assert_eq!(result, 21);

        file = String::new();
        file.push_str("3037345\n");  
        file.push_str("2551255\n");
        file.push_str("6533255\n");
        file.push_str("3354966\n");
        file.push_str("3539077");

        let result: usize = solution::solve(file);
        assert_eq!(result, 23);
    }
}

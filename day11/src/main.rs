use std::fs;

fn main() {
    let file = fs::read_to_string("./input").expect("file is present");
    let result = solution::solve(file);
    println!("{}", result);
}

mod solution {

    // part one
    use std::collections::HashMap;

    pub fn solve(file: String) -> usize {        
        let content = file.lines();

        let mut monkeys: Vec<HashMap<&str, Vec<usize>>> = Vec::new();
        let mut new_monkey = HashMap::<&str, Vec<usize>>::new();

        // parse input
        for l in content {
            let cmds = l.split(' ').collect::<Vec<&str>>();
            if cmds.contains(&"Monkey") {
                new_monkey.insert("number", vec![monkeys.len()]);

            } else if cmds.contains(&"items:") {
                let mut items = Vec::<usize>::new();
                for i in 4..cmds.len() {
                    let item = cmds[i].replace(',', "").parse::<usize>().unwrap();
                    items.push(item);
                }

                new_monkey.insert("items", items);

            } else if cmds.contains(&"Operation:") {
                let operation = cmds[6];
                let operation_amount: usize;
                if cmds[7] == "old" {
                    operation_amount = 9999999;
                } else {
                    operation_amount = cmds[7].parse::<usize>().unwrap();
                }

                new_monkey.insert("operation", vec![if operation == "*" { 0 } else { 1 }, operation_amount]);

            } else if cmds.contains(&"Test:") {
                let divisible = cmds[5].parse::<usize>().unwrap();
                new_monkey.insert("divisible", vec![divisible]);
            
            } else if cmds.contains(&"throw") {
                let throw = cmds[9].parse::<usize>().unwrap();
                if new_monkey.contains_key("true") {
                    new_monkey.insert("false", vec![throw]);
                    continue;
                }
                new_monkey.insert("true", vec![throw]);
            
            } else {
                monkeys.push(new_monkey.clone());
                new_monkey.clear();
            }
        }
        
        monkeys.push(new_monkey.clone());
        new_monkey.clear();

        let mut operations: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        
        // rounds
        for _ in 0..20 {
            let mut edit_m: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();
            for m in &monkeys {

                let operation = m["operation"][0];
                let op_amount = m["operation"][1];
                let divisible = m["divisible"][0];

                for i in 0..m["items"].len() {
                    do_operations(m["number"][0], m["items"][i], (m["true"][0], m["false"][0]), operation, m["items"][i], op_amount, divisible, &mut edit_m, &mut operations);
                }

                let mut i = 0;
                while edit_m.len() > 0 {
                    if edit_m[i].0 == m["number"][0] {
                        do_operations(edit_m[i].0, edit_m[i].1, (m["true"][0], m["false"][0]), operation, edit_m[i].1, op_amount, divisible, &mut edit_m, &mut operations);
                        edit_m.remove(i);
                    } else {

                        i += 1;
                        if i >= edit_m.len() {
                            break;
                        }
                    }
                }
            }

            for k in 0..monkeys.len() {
                monkeys[k].get_mut("items").unwrap().clear();
            }

            for i in 0..edit_m.len() {
                monkeys[edit_m[i].0].get_mut("items").unwrap().push(edit_m[i].1);
            }
            edit_m.clear();
        }

        println!("{:?}", operations);
        let mut biggest: (usize, usize) = (0, 0);
        for o in operations {
            if o > biggest.1 {
                biggest.0 = biggest.1;
                biggest.1 = o;
            } else if o > biggest.0 {
                biggest.0 = o;
            }
        }

        biggest.0 * biggest.1
    }

    fn do_operations(
        number: usize,
        item: usize, 
        (t, f): (usize, usize), 
        operation: usize, 
        mut monkey_wl: usize, 
        mut op_amount: usize, 
        divisible: usize,
        edit_m: &mut Vec<(usize, usize)>,
        operations: &mut Vec<usize>
    ) {
        if op_amount == 9999999 {
            op_amount = item;
        }

        if operation == 0 {
            monkey_wl = item * op_amount;

        } else {
            monkey_wl = item + op_amount;
        }
        
        monkey_wl /= 3;

        let mut monkey_to_edit: usize = 0;

        if monkey_wl % divisible == 0 {
            monkey_to_edit = t;
        } else {
            monkey_to_edit = f;
        }
        edit_m.push((monkey_to_edit, monkey_wl));

        operations[number] += 1;
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
        
        assert_eq!(result, 10605);
    }
}

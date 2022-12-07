use std::fs;
use crate::stack::Stack;

mod stack;

fn main() {
    let content = fs::read_to_string("./input").expect("file should be ok");
    let lines = content.lines();
    let clines = lines.clone();
    
    let mut stacks: Vec<Stack<&str>> = Vec::<Stack<&str>>::new();
    for i in lines.enumerate() {
        let boxes: Vec<&str> = i.split(' ').collect();
        
        for b in 0..8 {
            println!("{}", boxes[b]);
            if stacks[b].height() == 0 {
                let new_stack = Stack::<&str>::new();
                stacks[b] = new_stack;
            }

            stacks[b].push(boxes[b]);
        }
    }

    for i in clines.skip(10) {
        println!("{}", i);
        let instructions: Vec<&str> = i.split(' ').collect();
        let counts = instructions[1].parse::<i32>().unwrap();
        for _ in 0..counts {
            let stack_num_pop = instructions[3].parse::<usize>().expect("should be a number");
            let stack_num_add = instructions[5].parse::<usize>().expect("should be a number");
            let popped = stacks[stack_num_pop + 1].pop().unwrap();
            stacks[stack_num_add + 1].push(popped)
        }
    }

    for i in 7..0 {
        println!("{:?}", stacks[i].top())
    }
}

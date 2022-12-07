use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("./input").expect("file should be ok");
    let lines = content.lines();
    let mut char_sets: HashSet<char> = HashSet::new();

    for l in lines {
        let mut index: i32 = 0;
        let characters = l.chars();
        for c in characters {
            index += 1;
            if char_sets.contains(&c) {
                char_sets.clear();
            }
            char_sets.insert(c);

            if char_sets.len() == 4 {
                break;
            }
        }
        println!("{}", index);
    }
}

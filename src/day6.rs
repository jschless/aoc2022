use std::collections::HashSet;
use std::fs;

fn main() {
    day6();
}

fn day6() {
    let code = fs::read_to_string("./input/day6.txt")
        .expect("Could not read input")
        .chars()
        .collect::<Vec<char>>();
    let mut flag = true;
    for i in 0..code.len() - 13 {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        for j in 0..14 {
            if j < 4 {
                set1.insert(&code[i + j]);
            }
            set2.insert(&code[i + j]);
        }
        if flag && set1.len() == 4 {
            println!("{:?}", set1);
            println!("The start-of-packet marker is at location {}", i + 4);
            flag = false;
        }

        if set2.len() == 14 {
            println!("{:?}", set2);
            println!("The start-of-message marker is at location {}", i + 14);
        }
    }
}

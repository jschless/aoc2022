use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    day3();
}

fn day3() {
    let input = fs::read_to_string("./input/day3.txt").expect("Could not read input");

    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut char_to_int = HashMap::new();

    let mut char_val = 1;
    for c in chars.chars() {
        char_to_int.insert(c, char_val);
        char_val += 1;
    }

    let mut acc = 0;
    for rucksack in input.split("\n") {
        let (r1, r2) = rucksack.split_at(rucksack.len() / 2);
        let set1: HashSet<char, RandomState> = HashSet::from_iter(r1.chars());
        let set2: HashSet<char, RandomState> = HashSet::from_iter(r2.chars());
        for x in set1.intersection(&set2) {
            acc += char_to_int.get(x).unwrap();
        }
    }
    println!("done, value is  {}", acc);

    let lines: Vec<Vec<char>> = input
        .split("\n")
        .map(|s| s.to_string().chars().collect())
        .collect();
    acc = 0;
    for i in (0..lines.len()).step_by(3) {
        let (s1, s2, s3) = (&lines[i], &lines[i + 1], &lines[i + 2]);
        for c in s1 {
            if s2.contains(c) && s3.contains(c) {
                acc += char_to_int.get(c).unwrap();
                break;
            }
        }
    }
    println!("done, value is  {}", acc);
}

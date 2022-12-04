use std::fs;

fn main() {
    day1();
}

fn day1() {
    let elves_contents = fs::read_to_string("./input/day1.txt").expect("Could not read input");

    let mut calorie_counts = elves_contents
        .split("\n\n")
        .map(|s| s.split("\n").map(|x| x.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    calorie_counts.sort();
    calorie_counts.reverse();

    println!("Largest calorie count: {}", calorie_counts[0]);

    let top3 = &calorie_counts[0..3];
    println!("Top 3 calorie counts: {}", top3.iter().sum::<i32>());
}

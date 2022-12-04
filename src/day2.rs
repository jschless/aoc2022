use std::fs;

fn main() {
    day2();
}

fn day2() {
    let strategy = fs::read_to_string("./input/day2.txt").expect("Could not read input");

    let mut acc1 = 0;
    let mut acc2 = 0;
    for round in strategy.split("\n") {
        // let (opponent_move, my_move) = round.split_at(1);
        let (a, b) = round.split_at(1);
        acc1 += match (a, b) {
            ("A", " X") => 4,
            ("A", " Y") => 8,
            ("A", " Z") => 3,
            ("B", " X") => 1,
            ("B", " Y") => 5,
            ("B", " Z") => 9,
            ("C", " X") => 7,
            ("C", " Y") => 2,
            ("C", " Z") => 6,
            _ => 0,
        };
        let temp2 = match (a, b) {
            ("A", " X") => 3,
            ("A", " Y") => 4,
            ("A", " Z") => 8,
            ("B", " X") => 1,
            ("B", " Y") => 5,
            ("B", " Z") => 9,
            ("C", " X") => 2,
            ("C", " Y") => 6,
            ("C", " Z") => 7,
            _ => 0,
        };
        acc2 += temp2;
    }
    println!("Final score, part 1: {}", acc1);
    println!("Final score, part 2: {}", acc2);
}

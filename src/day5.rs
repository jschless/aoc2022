use std::fs;

fn main() {
    day5();
}

fn day5() {
    let stacks = fs::read_to_string("./input/day5.txt").expect("Could not read input");

    let parsed = stacks
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..9 {
        stacks.push(Vec::new());
    }

    let stacks_txt = parsed[0].replace("[", " ").replace("]", " "); // remove this bs

    for line in stacks_txt.split("\n") {
        for (i, s) in line.chars().enumerate() {
            if s.is_numeric() {
                // we've reached the index row
                break;
            }
            if s != ' ' {
                // we have an actual character, use its index to determine which stack it belongs in
                stacks[(i - 1) / 4].insert(0, s);
            }
        }
    }

    let mut stacks_2 = stacks.clone();
    for instruction in parsed[1].split("\n") {
        let words = instruction.split(" ").collect::<Vec<&str>>();
        let number = words[1].parse::<usize>().unwrap();
        let source = words[3].parse::<usize>().unwrap();
        let dest = words[5].parse::<usize>().unwrap();

        for _i in 0..number {
            //part 1
            let temp = stacks[source - 1].pop().unwrap();
            stacks[dest - 1].push(temp);
        }

        let mut temp_vec: Vec<char> = Vec::new(); // part 2
        for _i in 0..number {
            let temp = stacks_2[source - 1].pop().unwrap();
            temp_vec.insert(0, temp);
        }
        for c in temp_vec.iter() {
            stacks_2[dest - 1].push(*c);
        }
    }
    print!("Part 1: ");
    for s in stacks {
        print!("{}", s.last().unwrap());
    }

    print!("\nPart 2: ");
    for s in stacks_2 {
        print!("{}", s.last().unwrap());
    }
}

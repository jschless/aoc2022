use std::collections::HashSet;
use std::fs;

fn main() {
    day10();
}

fn check(
    imp_cycles: &HashSet<i64>,
    register: &i64,
    cycle: &i64,
    pixel_collecter: &mut Vec<char>,
) -> i64 {
    if *register >= (*cycle % 40) - 2 && *register <= (*cycle % 40) {
        pixel_collecter.push('#');
    } else {
        pixel_collecter.push('.');
    }

    if imp_cycles.contains(&cycle) {
        return cycle * register;
    }
    return 0;
}

fn day10() {
    let signal = fs::read_to_string("./input/day10.txt").expect("Could not read input");
    let mut cycle: i64 = 0;
    let mut acc: i64 = 0;
    let mut register: i64 = 1;
    let imp_cycles = HashSet::from([20, 60, 100, 140, 180, 220]);
    let pixel_collecter: &mut Vec<char> = &mut Vec::new();
    for instr in signal.split("\n") {
        cycle += 1; //noop
        acc += check(&imp_cycles, &register, &cycle, pixel_collecter);
        if instr.starts_with("addx") {
            let (_, val) = instr.split_at(4);
            cycle += 1;
            acc += check(&imp_cycles, &register, &cycle, pixel_collecter);
            register += val.trim().parse::<i64>().unwrap();
        }
    }

    for (i, c) in (*pixel_collecter).iter().enumerate() {
        // part 2 print out
        if i % 40 == 0 {
            println!("");
        }
        print!("{}", c);
    }
    println!("The sum of signal strengths is {}", acc);
}

use std::fs;

fn main() {
    day4();
}

fn day4() {
    let pairs = fs::read_to_string("./input/day4.txt").expect("Could not read input");
    let mut acc = 0;
    let mut acc2 = 0;
    for pair in pairs.split("\n") {
        let elves = pair.split(",").collect::<Vec<&str>>();
        let elf1 = elves[0].split("-").collect::<Vec<&str>>();
        let e11 = elf1[0].parse::<i32>().unwrap();
        let e12 = elf1[1].parse::<i32>().unwrap();
        let elf2 = elves[1].split("-").collect::<Vec<&str>>();
        let e21 = elf2[0].parse::<i32>().unwrap();
        let e22 = elf2[1].parse::<i32>().unwrap();
        if (e11 <= e21 && e12 >= e22) || (e21 <= e11 && e22 >= e12) {
            acc += 1;
        }
        if ((e11 <= e21 && e12 >= e21) || (e11 <= e22 && e12 >= e22))
            || ((e21 <= e11 && e22 >= e11) || (e21 <= e12 && e22 >= e12))
        {
            acc2 += 1;
        }
    }
    println!("Number of encompassing ranges: {}", acc);
    println!("Number of overlapping ranges: {}", acc2);
}

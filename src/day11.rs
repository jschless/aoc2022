use std::collections::HashMap;
use std::fs;
fn main() {
    day11();
}

fn get_last_word(line: &str) -> &str {
    return line
        .split(" ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .trim();
}

struct Monkey {
    // operation: Box<dyn Fn(usize) -> usize>,
    modulus: usize,
    true_monkey: usize,
    false_monkey: usize,
}

fn monkey_operation(monkey: usize, item: &usize) -> usize {
    // i originally parsed the operation for each monkey as a closure, but then I couldn't
    // figure out how to give a closure to each monkey and actually call it.
    return match monkey {
        0 => item * 19,
        1 => item + 8,
        2 => item * 13,
        3 => item + 6,
        4 => item + 5,
        5 => item * item,
        6 => item + 2,
        7 => item + 3,
        _ => *item,
    };
    // test example code
    // return match monkey {
    //     0 => item * 19,
    //     1 => item + 6,
    //     2 => item * item,
    //     3 => item + 3,
    //     _ => *item,
    // };
}

fn day11() {
    let monkeys = fs::read_to_string("./input/day11.txt").expect("Could not read input");
    let mut monkey_vec = Vec::new();
    let mut items: Vec<Vec<usize>> = Vec::new(); // separating the thing i'm going to be mutating from the struct
    let mut inspection_count = HashMap::new();
    for i in 0..8 {
        inspection_count.insert(i, 0);
    }
    for m in monkeys.split("\n\n") {
        // Parsing all this bullshit
        let lines = m.split("\n").collect::<Vec<&str>>();
        let starting_items = lines[1].split(":").collect::<Vec<&str>>()[1]
            .split(",")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let modulus = get_last_word(lines[3]).parse::<usize>().unwrap();
        let if_true = get_last_word(lines[4]).parse::<usize>().unwrap();
        let if_false = get_last_word(lines[5]).parse::<usize>().unwrap();
        monkey_vec.push(Monkey {
            modulus: modulus,
            true_monkey: if_true,
            false_monkey: if_false,
        });
        items.push(starting_items);
    }
    let mod_prod = monkey_vec
        .iter()
        .map(|m| m.modulus)
        .fold(1, |res, a| res * a);

    for _r in 0..10000 {
        for i in 0..monkey_vec.len() {
            let m = &monkey_vec[i];
            let cur_monkey_item_list = items[i].clone();
            for item in cur_monkey_item_list {
                inspection_count.entry(i).and_modify(|x| *x += 1);
                let mut new_item = monkey_operation(i, &item);
                new_item = new_item % mod_prod;
                if new_item % m.modulus == 0 {
                    let _ = &items[m.true_monkey].push(new_item);
                } else {
                    let _ = &items[m.false_monkey].push(new_item);
                }
            }
            items[i] = Vec::new();
        }
        // println!("After round {}, \n {:?}", _r, items);
    }
    let mut counts = inspection_count.iter().collect::<Vec<(&usize, &usize)>>();
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    let total: usize = counts[..2].iter().map(|x| x.1).fold(1, |res, a| res * a);
    // println!("{:?}", counts);
    println!("the two most active monkeys product is {}", total);
}

use std::collections::HashSet;
use std::fs;
fn main() {
    day9();
}

fn day9() {
    let moves = fs::read_to_string("./input/day9.txt").expect("Could not read input");
    const ROPE_LEN: usize = 10;
    let mut tail_locations: HashSet<[i64; 2]> = HashSet::new();
    let mut rope: Vec<[i64; 2]> = Vec::new();
    for _ in 0..ROPE_LEN {
        rope.push([0, 0]);
    }
    for m in moves.split("\n") {
        let (dir, len) = m.split_at(1);
        // start by moving the head
        for _ in 0..len.to_string().trim().parse::<i32>().unwrap() {
            let mut new_head = rope[0];
            match dir {
                "L" => new_head[0] -= 1,
                "R" => new_head[0] += 1,
                "U" => new_head[1] += 1,
                "D" => new_head[1] -= 1,
                _ => println!("Received weird input: {}", dir),
            }
            rope[0] = new_head;
            // move the rest of the rope accordingly
            for i in 1..ROPE_LEN {
                let lead = rope[(i as usize) - 1];
                let mut follow = rope[(i as usize)];
                for (i1, i2) in [(0, 1), (1, 0)] {
                    for flag in [-1, 1] {
                        // remove code replication
                        if lead[i2] - follow[i2] == flag * 2 {
                            // head is two away in some axis
                            follow[i2] += flag; // correct this difference
                            if flag * (lead[i1] - follow[i1]) > 0 {
                                // check for difference in other axis and correct
                                follow[i1] += flag;
                            } else if flag * (lead[i1] - follow[i1]) < 0 {
                                follow[i1] -= flag;
                            }
                        }
                    }
                }
                rope[(i as usize)] = follow;
            }
            tail_locations.insert(rope[ROPE_LEN - 1]);
        }
    }

    println!("Tail has visited {} locations", tail_locations.len());
}

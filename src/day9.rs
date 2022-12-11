use std::collections::HashSet;
use std::fs;
fn main() {
    day9();
}

fn day9() {
    let moves = fs::read_to_string("./input/day9.txt").expect("Could not read input");
    let mut tail_locations: HashSet<(i64, i64)> = HashSet::new();
    let mut rope: Vec<(i64, i64)> = Vec::new();
    const ROPE_LEN: usize = 10;
    for _ in 0..ROPE_LEN {
        rope.push((0, 0));
    }
    for m in moves.split("\n") {
        let (dir, len) = m.split_at(1);
        // start by moving the head
        for _ in 0..len.to_string().trim().parse::<i32>().unwrap() {
            let mut new_head = rope[0];
            match dir {
                "L" => new_head.0 -= 1,
                "R" => new_head.0 += 1,
                "U" => new_head.1 += 1,
                "D" => new_head.1 -= 1,
                _ => println!("Received weird input: {}", dir),
            }
            rope[0] = new_head;
            for i in 1..ROPE_LEN {
                let lead = rope[(i as usize) - 1];
                let mut follow = rope[(i as usize)];
                if lead.1 - follow.1 == 2 {
                    // head is two up of tail
                    follow.1 += 1; // default, no diagonal
                    if lead.0 - follow.0 > 0 {
                        // head is  right of tail
                        follow.0 += 1;
                    } else if lead.0 - follow.0 < 0 {
                        //head is left of tail
                        follow.0 -= 1;
                    }
                }
                if lead.1 - follow.1 == -2 {
                    // head is two down of tail
                    follow.1 -= 1;
                    if lead.0 - follow.0 > 0 {
                        follow.0 += 1; // move diagonally (down and right)
                    } else if lead.0 - follow.0 < 0 {
                        follow.0 -= 1;
                    }
                }
                if lead.0 - follow.0 == 2 {
                    // head is two right of tail
                    follow.0 += 1; // move right
                    if lead.1 - follow.1 > 0 {
                        // head is one up of tail
                        follow.1 += 1; // move diagonally (up and right)
                    } else if lead.1 - follow.1 < 0 {
                        //head is one down of tail
                        follow.1 -= 1;
                    }
                }
                if lead.0 - follow.0 == -2 {
                    // head is two left  of tail
                    follow.0 -= 1; // move left
                    if lead.1 - follow.1 > 0 {
                        // head is one up of tail
                        follow.1 += 1; // move diagonally (up and left)
                    } else if lead.1 - follow.1 < 0 {
                        //head is one down of tail
                        follow.1 -= 1;
                    }
                }
                rope[(i as usize)] = follow;
            }
            tail_locations.insert(rope[ROPE_LEN - 1]);
        }
    }

    println!("Tail has visited {} locations", tail_locations.len());
    // println!("{:?}", tail_locations);
}

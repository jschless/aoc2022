use std::collections::HashSet;
use std::fs;
fn main() {
    day9();
}

fn day9() {
    let moves = fs::read_to_string("./input/day9.txt").expect("Could not read input");
    let mut tail_locations: HashSet<(i64, i64)> = HashSet::from([(0, 0)]);
    let (mut h_i, mut h_j) = (0, 0);
    let (mut t_i, mut t_j) = (0, 0);
    for m in moves.split("\n") {
        let (dir, len) = m.split_at(1);
        for _ in 0..len.to_string().trim().parse::<i32>().unwrap() {
            match dir {
                "L" => h_i -= 1,
                "R" => h_i += 1,
                "U" => h_j += 1,
                "D" => h_j -= 1,
                _ => println!("Received weird input: {}", dir),
            }
            if h_j - t_j == 2 {
                // head is two up of tail
                t_j += 1; // default, no diagonal
                if h_i - t_i == 1 {
                    // head is one right of tail
                    t_i += 1;
                } else if h_i - t_i == -1 {
                    //head is one left of tail
                    t_i -= 1;
                }
            }
            if h_j - t_j == -2 {
                // head is two down of tail
                t_j -= 1;
                if h_i - t_i == 1 {
                    t_i += 1; // move diagonally (down and right)
                } else if h_i - t_i == -1 {
                    t_i -= 1;
                }
            }
            if h_i - t_i == 2 {
                // head is two right of tail
                t_i += 1; // move right
                if h_j - t_j == 1 {
                    // head is one up of tail
                    t_j += 1; // move diagonally (up and right)
                } else if h_j - t_j == -1 {
                    //head is one down of tail
                    t_j -= 1;
                }
            }
            if h_i - t_i == -2 {
                // head is two left  of tail
                t_i -= 1; // move left
                if h_j - t_j == 1 {
                    // head is one up of tail
                    t_j += 1; // move diagonally (up and left)
                } else if h_j - t_j == -1 {
                    //head is one down of tail
                    t_j -= 1;
                }
            }
            tail_locations.insert((t_i, t_j));
        }
    }

    println!("Tail has visited {} locations", tail_locations.len());
    // println!("{:?}", tail_locations);
}

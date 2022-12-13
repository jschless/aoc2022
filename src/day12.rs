use std::collections::HashSet;
use std::fs;

fn main() {
    day12();
}

fn get_neighbors(coord: (i32, i32), max_dim_y: i32, max_dim_x: i32) -> Vec<(i32, i32)> {
    let (i, j) = coord;
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if !(i + x >= (max_dim_x as i32) || i + x < 0 || j + y >= (max_dim_y as i32) || j + y < 0) {
            neighbors.push((i + x, j + y));
        }
    }
    return neighbors;
}
fn get_value(coord: &(i32, i32), map: &Vec<Vec<i32>>) -> i32 {
    return map[coord.0 as usize][coord.1 as usize];
}
fn day12() {
    let map = fs::read_to_string("./input/day12.txt").expect("Could not read input");
    let mut parsed_map = map
        .split("\n")
        .map(|line| {
            // converting chars to ints
            line.as_bytes()
                .iter()
                .map(|i| (*i) as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    const ENDPOINT: i32 = 69;
    const STARTPOINT: i32 = 83;

    // find where the S is
    let mut starts: Vec<(i32, i32)> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..parsed_map.len() {
        for j in 0..parsed_map[0].len() {
            if get_value(&(i as i32, j as i32), &parsed_map) == STARTPOINT {
                start = (i as i32, j as i32);
                starts.push(start);
            }
            if get_value(&(i as i32, j as i32), &parsed_map) == ENDPOINT {
                end = (i, j);
            }
            if get_value(&(i as i32, j as i32), &parsed_map) == 97 {
                starts.push((i as i32, j as i32));
            }
        }
    }
    // adjusting the S and E to be the proper number
    parsed_map[end.0][end.1] = 122;
    parsed_map[start.0 as usize][start.1 as usize] = 97;
    let max_y = parsed_map.len();
    let max_x = parsed_map[0].len();
    let mut min_moves = 100000000;
    for s in starts {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut to_explore: Vec<Vec<(i32, i32)>> = Vec::new();
        to_explore.push(Vec::from([s]));
        while to_explore.len() > 0 {
            let current_path: Vec<(i32, i32)> = to_explore.remove(0);
            let current_index = current_path.iter().last().unwrap();
            if current_index.0 == end.0 as i32
                && current_index.1 == end.1 as i32
                && current_path.len() < min_moves
            {
                min_moves = current_path.len();
                break;
            }
            // add neighbors
            for n in get_neighbors(*current_index, max_x as i32, max_y as i32) {
                if get_value(&n, &parsed_map) - get_value(&current_index, &parsed_map) <= 1
                    && !current_path.contains(&n)
                    && !visited.contains(&n)
                {
                    let mut next_path = current_path.clone();
                    next_path.push(n);
                    visited.insert(n.clone());
                    to_explore.push(next_path);
                }
            }
        }
    }
    println!("It took {} moves to get there", min_moves - 1);
}

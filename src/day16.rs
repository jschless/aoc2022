use std::collections::HashMap;
use std::fs;

fn main() {
    day16();
}

fn day16() {
    let valves = fs::read_to_string("./input/day16test.txt").expect("Could not read input");
    let mut valve_to_int: HashMap<String, usize> = HashMap::new();
    let mut neighbors: Vec<Vec<String>> = Vec::new();
    let mut flow_rates: Vec<i32> = Vec::new();
    let mut pressurized: Vec<bool> = Vec::new();
    for (i, line) in valves.split("\n").enumerate() {
        valve_to_int.insert(&line[6..8].to_string(), i);

        flow_rates.push(
            *(&line[line.find("flow rate=").unwrap() + 10..line.find(";").unwrap()]
                .parse::<i32>()
                .unwrap()),
        );

        let valves = &line[line.find("to valve").unwrap() + 9..].trim();
        if valves.chars().count() == 2 {
            neighbors.push(Vec::from([valves.to_string()]));
        } else {
            neighbors.push(
                valves
                    .split(", ")
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>(),
            );
        }
        pressurized.push(false);
    }
}

fn movement(path: Vec<usize>, acc_pressure: i32, pressure_count: i32, time: usize, neighbors: <Vec<String>>, flow_rates: Vec<i32>, valve_to_int:HashSet<String, i32>, pressurized: Vec<usize>) -> i32 {
    let cur_node = path.last();
    if time == 0 { // time is up
        return acc_pressure + pressure_count;
    } else {
        let results = Vec::new();
        if !pressurized[cur_node] {
            movement(path, acc_pressure + pressure_count, time+1, neighbors, flow_rates, valve_to_int,)

        }
        movement(path, acc_pressure + pressure_count, time+1, neighbors, flow_rates, valve_to_int,)
    }
}

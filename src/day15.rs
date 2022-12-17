use std::collections::HashSet;
use std::fs;

fn main() {
    day15();
}

fn day15() {
    let beacons = fs::read_to_string("./input/day15.txt").expect("Could not read input");
    let mut sens: Vec<(i32, i32)> = Vec::new();
    let mut beac: Vec<(i32, i32)> = Vec::new();
    for b in beacons.split("\n") {
        let mut coords: Vec<(i32, i32)> = Vec::new();
        for s in b.split(":") {
            let sensor_loc_x = s.find("x=").unwrap();
            let sensor_loc_y = s.find("y=").unwrap();
            // println!("{} {}", s, &s[sensor_loc_x + 2..sensor_loc_y - 2]);
            let x_loc = &s[sensor_loc_x + 2..sensor_loc_y - 2]
                .parse::<i32>()
                .unwrap();
            let y_loc = &s[sensor_loc_y + 2..].trim().parse::<i32>().unwrap();
            // println!("{} {}", x_loc, y_loc);
            coords.push((*x_loc, *y_loc))
        }
        sens.push(coords[0]);
        beac.push(coords[1]);
    }
    let mut clear_locs: HashSet<(i32, i32)> = HashSet::new();
    let mut radii: Vec<i32> = Vec::new();
    let mut low_x = 100000000;
    let mut high_x = 0;
    let mut ranges: Vec<Vec<i32>> = Vec::new();
    for i in 0..(sens.len()) {
        // println!("exploring sensor {}", i);
        let b = beac[i];
        let s = sens[i];
        let dist = manhattan_distance(b, s);
        radii.push(dist);
        ranges.push(Vec::from([b.0 - dist, b.0 + dist, b.1 - dist, b.1 + dist]));
        if s.0 - dist < low_x {
            low_x = s.0 - dist;
        }
        if s.0 + dist > high_x {
            high_x = s.0 + dist;
        }
    }
    let magic_num = 2000000;

    let mut coverage = Vec::new();
    for i in 0..(sens.len()) {
        let d = radii[i];
        let s = sens[i];
        let dy = (magic_num - s.1).abs();
        if dy < d {
            // we overlap with the row of interest
            let dx = d - dy;
            coverage.push((s.0 - dx, s.0 + dx));
        }
    }
    // collapse ranges
    coverage.sort();
    println!("{:?}", coverage);
    let mut condensed_coverage = Vec::new();
    let mut low = coverage[0].0;
    let mut high = coverage[0].1;
    for c in &coverage[1..] {
        if c.0 < high && c.1 > high && c.0 >= low {
            // if they overlap, just make the upper limit the larger one;
            high = c.1;
        } else if c.0 > high {
            // start new range as long as it's not contained by the previous one.
            // if they don't overlap, just add the range and start over
            condensed_coverage.push((low, high));
            low = c.0;
            high = c.1;
        }
        // println!("{} {} {}", low, high, high - low);
    }
    condensed_coverage.push((low, high));

    println!(
        "{}",
        condensed_coverage.iter().map(|t| t.1 - t.0).sum::<i32>()
    );

    for b in beac {
        if clear_locs.contains(&b) {
            clear_locs.remove(&b);
        }
    }
    // part 1: 4873353
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
}

use std::cmp::max;
use std::cmp::min;
use std::fs;

fn main() {
    day14();
}

fn day14() {
    let rock_lines = fs::read_to_string("./input/day14.txt")
        .expect("Could not read input")
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|x| {
                    x.split(",")
                        .map(|a| a.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();
    let x_adjust = 200; // making room for the infinite floor
    let max_x = rock_lines
        .iter()
        .map(|x| x.iter().map(|y| y[0]).max().unwrap())
        .max()
        .unwrap()
        + x_adjust;
    let max_y = rock_lines
        .iter()
        .map(|x| x.iter().map(|y| y[1]).max().unwrap())
        .max()
        .unwrap()
        + 2;

    let mut cave: Vec<Vec<usize>> = Vec::new();
    for _i in 0..(max_x + 1) {
        let mut temp = Vec::new();
        for _j in 0..(max_y + 1) {
            temp.push(0);
        }
        cave.push(temp);
    }
    let mut start: Vec<usize>;
    let mut end: Vec<usize>;
    for line in rock_lines.iter() {
        for i in 0..line.len() - 1 {
            start = line[i].clone();
            end = line[i + 1].clone();
            for point in find_points_on_line(start, end) {
                cave[point[0]][point[1]] = 3;
            }
        }
    }
    for i in 0..(max_x + 1) {
        // adding cave floor
        cave[i][max_y] = 3;
    }
    let mut sand_in_abyss = false;
    let mut n_sand = 0;
    while !sand_in_abyss {
        n_sand += 1;
        let mut sand_at_rest = false;
        let mut sand = Vec::from([500, 0]);
        while !sand_at_rest {
            if cave[500][0] == 3 || sand[0] >= max_x || sand[0] < 0 || sand[1] >= max_y {
                sand_at_rest = true;
                sand_in_abyss = true;
            } else if cave[sand[0]][sand[1] + 1] == 0 {
                sand = Vec::from([sand[0], sand[1] + 1]);
            } else if cave[sand[0] - 1][sand[1] + 1] == 0 {
                sand = Vec::from([sand[0] - 1, sand[1] + 1]);
            } else if cave[sand[0] + 1][sand[1] + 1] == 0 {
                sand = Vec::from([sand[0] + 1, sand[1] + 1]);
            } else {
                sand_at_rest = true;
                cave[sand[0]][sand[1]] = 3;
            }
        }
    }
    println!("number of sand that fell: {}", n_sand - 1);
}

fn find_points_on_line(start: Vec<usize>, end: Vec<usize>) -> Vec<Vec<usize>> {
    let mut rock_points: Vec<Vec<usize>> = Vec::new();
    let low_x = min(start[0], end[0]);
    let high_x = max(start[0], end[0]);
    let low_y = min(start[1], end[1]);
    let high_y = max(start[1], end[1]);

    for x in low_x..(high_x + 1) {
        for y in low_y..(high_y + 1) {
            rock_points.push(Vec::from([x, y]));
        }
    }
    return rock_points;
}

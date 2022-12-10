use std::collections::HashMap;
use std::fs;

fn main() {
    day7();
}

fn extract_dir(command: &str) -> &str {
    let temp: &str = command.split(" ").collect::<Vec<&str>>().last().unwrap();
    return temp.trim();
}

fn day7() {
    let mut wdir: Vec<&str> = Vec::new();
    let mut dir_components: HashMap<String, Vec<String>> = HashMap::new();
    let mut dir_depths = Vec::new();
    let code = fs::read_to_string("./input/day7.txt").expect("Could not read input");
    for line in code.split("$").map(|x| x.trim()).filter(|x| !x.is_empty()) {
        if line.starts_with("cd") {
            let dir = extract_dir(&line);
            if dir == ".." {
                // go one directory back
                wdir.pop();
            } else if dir == "/" {
                // go to root
                wdir = Vec::from(["/"]);
                let wdir_str = wdir.join("/");
                dir_depths.push((wdir_str.clone(), wdir.len()));
            } else {
                wdir.push(dir);
                let wdir_str = wdir.join("/");
                dir_depths.push((wdir_str.clone(), wdir.len()));
            }
        }
        if line.starts_with("ls") {
            let wdir_str = wdir.join("/");
            let mut components = Vec::new();
            for file in line.split("\n").skip(1) {
                let temp = file.split(" ").collect::<Vec<&str>>();
                if temp[0].trim() == "dir" {
                    components.push(wdir_str.clone() + "/" + temp[1].trim()); // need full path because many directories are same name
                } else {
                    components.push(temp[0].trim().to_string()); // add file size
                }
            }
            dir_components.insert(wdir_str.clone(), components);
        }
    }
    dir_depths.sort_by(|a, b| b.1.cmp(&a.1));
    let mut dir_sizes: HashMap<String, i64> = HashMap::new();
    for (dir, _) in dir_depths.iter() {
        let files = dir_components.get(dir).unwrap();
        let mut acc = 0;
        for f in files {
            if f.parse::<i64>().is_ok() {
                // this is a file
                acc += f.parse::<i64>().unwrap();
            } else {
                // this is a directory, look up directory size
                acc += dir_sizes.get(&f.to_string()).unwrap();
            }
        }
        dir_sizes.insert(dir.to_string(), acc);
    }
    let mut acc = 0;
    let mut smallest_dir = 1000000000; // arbitrary large number
    let space_available = 70000000 - dir_sizes.get("/").unwrap();
    for (_, size) in &dir_sizes {
        if space_available + size >= 30000000 && smallest_dir > *size {
            smallest_dir = *size;
        }

        if size <= &100000 {
            acc += size;
        }
    }
    println!("The total size of directories <= 100000 is {}", acc);
    println!("The smallest dir you can delete has size {}", smallest_dir);
}

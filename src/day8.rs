use std::fs;

fn main() {
    day8();
}

fn day8() {
    let trees = fs::read_to_string("./input/day8.txt").expect("Could not read input");
    let mut treevec = Vec::new();
    let nrows = trees.matches("\n").count() + 1;
    let ncols = trees.find("\n").unwrap();

    for t in trees.chars().filter(|x| *x != '\n') {
        treevec.push(t.to_string().parse::<usize>().unwrap());
    }
    let mut acc = 0;
    for i in 0..ncols {
        for j in 0..nrows {
            let k = j * ncols + i;
            if i == 0 || j == 0 || i == ncols - 1 || j == ncols - 1 {
                acc += 1; // borders
            } else if ((1..(i + 1)).map(|x| treevec[k - x]).all(|x| treevec[k] > x))
                || ((1..(ncols - i))
                    .map(|x| treevec[k + x])
                    .all(|x| treevec[k] > x))
                || ((1..(j + 1))
                    .map(|x| treevec[k - ncols * x])
                    .all(|x| treevec[k] > x))
                || ((1..(nrows - j))
                    .map(|x| treevec[k + ncols * x])
                    .all(|x| treevec[k] > x))
            {
                acc += 1;
            }
        }
    }

    let mut best_score = 0;
    for i in 0..ncols {
        for j in 0..nrows {
            let k = j * ncols + i;
            let cur_val = treevec[k];

            let (mut lscore, mut rscore, mut dscore, mut uscore) = (0, 0, 0, 0);
            for x in 1..(i + 1) {
                if treevec[k - x] < cur_val {
                    lscore += 1;
                } else {
                    lscore += 1;
                    break;
                }
            }
            for x in 1..(ncols - i) {
                if treevec[k + x] < cur_val {
                    rscore += 1;
                } else {
                    rscore += 1;
                    break;
                }
            }
            for x in 1..(j + 1) {
                if treevec[k - ncols * x] < cur_val {
                    uscore += 1;
                } else {
                    uscore += 1;
                    break;
                }
            }
            for x in 1..(nrows - j) {
                if treevec[k + ncols * x] < cur_val {
                    dscore += 1;
                } else {
                    dscore += 1;
                    break;
                }
            }
            let temp_score = lscore * rscore * uscore * dscore;

            if temp_score > best_score {
                best_score = temp_score;
            }
        }
    }
    // println!("There are {} exterior candidates", acc1);
    // println!("There are {} interior candidates", acc2);
    println!("There are {} candidates", acc);
    println!("The best possible score is {} ", best_score);
}

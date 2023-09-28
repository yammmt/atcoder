// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut vin = vec![vec![]; h];
    for i in 0..h {
        input! {
            s: String,
        }
        vin[i] = s.chars().collect::<Vec<char>>();
    }
    // println!("{:?}", vin);

    let mut removed_w = vec![];
    for i in 0..w {
        let mut is_removed = false;
        for s in 0..h {
            if vin[s][i] == '#' {
                break;
            } else if s == h - 1 {
                is_removed = true;
            }
        }
        if is_removed {
            removed_w.push(i);
        }
    }

    for i in 0..h {
        if !(vin[i].iter().any(|&c| c == '#')) {
            continue;
        }

        for j in 0..w {
            if removed_w.contains(&j) {
                continue;
            }

            print!("{}", vin[i][j]);
        }
        println!("");
    }
}

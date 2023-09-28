// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut vrelations = vec![vec![]; n];
    for _i in 0..m {
        input! {
            x: usize,
            y: usize,
        }

        vrelations[x - 1].push(y - 1);
        vrelations[y - 1].push(x - 1);
    }

    let mut ans = 0;
    for bit_row in 1..2u32.pow(n as u32) {
        let mut selected = vec![];
        for i in 0..n {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        // println!("{:?}", selected);
        let mut is_valid = true;
        'outer: for i in 0..selected.len() {
            for j in i + 1..selected.len() {
                if !vrelations[selected[i]].contains(&selected[j]) {
                    // println!("{} doesn't know about {}", i, j);
                    is_valid = false;
                    break 'outer;
                }
            }
        }
        if is_valid {
            ans = ans.max(selected.len());
        }
    }

    println!("{}", ans);
}

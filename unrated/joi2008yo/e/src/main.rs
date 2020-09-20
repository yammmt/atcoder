// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[u8; c]; r],
    }

    let mut ans = 0;
    for bit_row in 0..2u32.pow(r as u32) {
        let mut selected = vec![];
        for i in 0..r {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        // println!("selected: {:?}", selected);

        let mut current_a = a.clone();
        for i in &selected {
            for j in 0..c {
                current_a[*i][j] = if current_a[*i][j] == 0 {
                    1
                } else {
                    0
                };
            }
        }
        // println!("{:?}", current_a);

        let mut current = 0;
        for i in 0..c {
            let mut one_num = 0;
            for j in 0..r {
                if current_a[j][i] == 0 {
                    continue;
                }

                one_num += 1;
            }

            if one_num > r / 2 {
                current += one_num;
            } else {
                current += r - one_num;
            }
        }
        ans = ans.max(current);
    }
    println!("{}", ans);
}

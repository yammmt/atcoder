// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: u32,
        w: u32,
        k: u32,
        c: [Chars; h],
    }

    let mut total_black: u32 = 0;
    for i in &c {
        total_black += i.into_iter().filter(|&a| a == &'#').count() as u32;
    }
    // println!("{}", total_black);

    let mut ans = 0;
    for bit_row in 0..2u32.pow(h) {
        // println!("bit_row: {}", bit_row);
        let mut red_rows = vec!();
        for i in 0..h {
            if bit_row & (1 << i) > 0 {
                red_rows.push(i);
            }
        }
        // println!("red_rows: {:?}", red_rows);
        for bit_col in 0..2u32.pow(w) {
            // println!("bit_col: {}", bit_col);
            let mut red_cols = vec!();
            for i in 0..w {
                if bit_col & (1 << i) > 0 {
                    red_cols.push(i);
                }
            }
            // println!("red_cols: {:?}", red_cols);

            let mut current_black = total_black;
            'i_loop:
            for i in 0..h {
                for j in 0..w {
                    if red_rows.contains(&i) || red_cols.contains(&j) {
                        if c[i as usize][j as usize] == '#' {
                            current_black -= 1;
                            if current_black < k {
                                break 'i_loop;
                            }
                        }
                    }
                }
            }
            if current_black == k {
                // println!("{}", current_black);
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

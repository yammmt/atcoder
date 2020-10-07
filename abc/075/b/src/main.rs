// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: isize,
        w: isize,
        mut c: [Chars; h as usize],
    }

    for i in 0..h {
        for j in 0..w {
            if c[i as usize][j as usize] != '.' {
                continue;
            }

            let mut bomb = 0;
            for hd in -1..2 {
                let hc = i + hd;
                if hc < 0 || hc >= h {
                    continue;
                }

                for wd in -1..2 {
                    let wc = j + wd;
                    if wc < 0 || wc >= w {
                        continue;
                    }

                    if c[hc as usize][wc as usize] == '#' {
                        bomb += 1;
                    }
                }
            }
            c[i as usize][j as usize] = std::char::from_digit(bomb, 10).unwrap();
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", c[i as usize][j as usize]);
        }
        println!();
    }
}

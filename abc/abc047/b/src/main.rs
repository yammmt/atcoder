// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
    }

    let mut is_white = vec![vec![true; w]; h];
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
            a: u8,
        }

        match a {
            1 => {
                for i in 0..h {
                    for j in 0..x {
                        is_white[i][j] = false;
                    }
                }
            },
            2 => {
                for i in 0..h {
                    for j in x..w {
                        is_white[i][j] = false;
                    }
                }
            },
            3 => {
                for i in 0..y {
                    for j in 0..w {
                        is_white[i][j] = false;
                    }
                }
            },
            4 => {
                for i in y..h {
                    for j in 0..w {
                        is_white[i][j] = false;
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    let ans = is_white.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&p| *p).count()
    });
    println!("{}", ans);
}

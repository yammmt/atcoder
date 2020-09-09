// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut tail = 0;
    let mut sharp_appeared = false;
    let mut dot_after_sharp = false;
    for i in 0..w {
        if a[0][i] == '#' {
            if dot_after_sharp {
                // like "##.#"
                println!("Impossible");
                return;
            }

            sharp_appeared = true;
            tail = tail.max(i as u32);
        } else {
            if sharp_appeared {
                dot_after_sharp = true;
            }
        }
    }

    for i in 1..h {
        let mut current_tail = 0;
        let mut sharp_appeared = false;
        dot_after_sharp = false;
        for j in 0..w {
            if a[i][j] == '#' {
                if (j as u32) < tail || dot_after_sharp {
                    println!("Impossible");
                    return;
                }

                sharp_appeared = true;
                current_tail = current_tail.max(j as u32);
            } else {
                if sharp_appeared {
                    dot_after_sharp = true;
                }
            }
        }
        tail = current_tail;
    }
    println!("Possible");
}
